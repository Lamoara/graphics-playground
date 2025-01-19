extern crate sdl2;
extern crate gl;

use std::{ffi::CString, fs::{self, File}, io::{self, BufRead}, time::Instant};

use cgmath::{perspective, Array, Deg, Matrix, Matrix4, Point3, Rad, Vector3};
use graphics_playground::fps_counter::FpsCounter;
use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};


fn main()
{
    // Inicializar SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Configurar el perfil de OpenGL
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3); // OpenGL 3.3
    gl_attr.set_depth_size(24);

    let window = video_subsystem
        .window("Ventana OpenGL", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Vértices de un cubo centrado en (0.0, 0.0, 0.0)
    let vertices = parse_obj("cube.obj").unwrap();


    let mut vbo = 0;
    let mut vao = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::Enable(gl::CULL_FACE); // Habilita el culling
        gl::CullFace(gl::BACK);    // Descarta las caras traseras
        gl::FrontFace(gl::CCW);    // Define las caras frontales como las de orden antihorario    

        gl::Enable(gl::DEPTH_TEST);  // Habilita el test de profundidad
        gl::DepthFunc(gl::LESS);     // Renderiza solo los fragmentos más cercanos
    }
    
    let vertex_shader_src = read_file_to_string("vertex_shader.glsl").unwrap();

    let fragment_shader_src = read_file_to_string("fragment_shader.glsl").unwrap();
        

    let shader_program = create_shader_program(vertex_shader_src.as_str(), fragment_shader_src.as_str());
    let start = Instant::now();

    let mut fps_counter = FpsCounter::new(60);

    let (mut pos_x, mut pos_y, mut pos_z): (f32, f32, f32) = (0.0, 0.0, 0.0);

    // Bucle principal
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop 
    {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {keycode, .. } => {
                    match keycode {
                        Some(keycode) => {
                            match keycode {
                                Keycode::Escape => break 'running,
                                Keycode::W => pos_z += 1.0,
                                Keycode::S => pos_z -= 1.0,
                                Keycode::A => pos_x += 1.0,
                                Keycode::D => pos_x -= 1.0,
                                Keycode::U => pos_y += 1.0,
                                Keycode::J => pos_y -= 1.0,
                                _ => ()
                            }
                        },
                        None => (),
                    }
                },
                _ => ()
            }
        }

        let position = create_position_matrix(Vector3::new(0.0, 0.0, 0.0));
        let rotation = create_rotation_matrix(0.0, start.elapsed().as_secs_f32() * 20.0, 0.0);
        let scale = create_scale_matrix(1.0);
        let model = scale * position * rotation;
        let view = create_view_matrix(Point3::new(pos_x, pos_y, pos_z), Vector3::new(0.0, 1.0, 0.0));
        let projection = create_projection_matrix(800.0/600.0);

        unsafe {
            gl::ClearColor(0.1, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(shader_program);
            set_uniform_matrix(shader_program, "model", &model);
            set_uniform_matrix(shader_program, "view", &view);
            set_uniform_matrix(shader_program, "projection", &projection);
            set_uniform_vec(shader_program, "cameraPos", &Vector3::new(pos_x, pos_y, pos_z));

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, (vertices.len()/3) as i32);
        }

        window.gl_swap_window();
        fps_counter.frame(true);
    }
}

fn compile_shader(src: &str, ty: gl::types::GLenum) -> u32 {
    let shader = unsafe { gl::CreateShader(ty) };
    let c_str = CString::new(src.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = Vec::with_capacity(len as usize);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut gl::types::GLchar,
            );
            panic!(
                "Shader compilation failed: {}",
                std::str::from_utf8(&buffer).ok().expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn create_shader_program(vertex_src: &str, fragment_src: &str) -> u32 {
    let vertex_shader = compile_shader(vertex_src, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_src, gl::FRAGMENT_SHADER);

    let program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Verificar errores de enlace
        let mut success = gl::FALSE as i32;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as i32 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut log = Vec::with_capacity(len as usize);
            log.set_len((len as usize) - 1); // Espacio para el \0
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);

            panic!(
                "Error vinculando el programa de shaders: {}",
                std::str::from_utf8(&log).unwrap()
            );
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    program
}

fn set_uniform_matrix(program: u32, name: &str, matrix: &cgmath::Matrix4<f32>) {
    let location = unsafe {
        let cname = CString::new(name).unwrap();
        gl::GetUniformLocation(program, cname.as_ptr())
    };

    unsafe {
        gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
    }
}

fn set_uniform_vec(program: u32, name: &str, vec: &Vector3<f32>) {
    let location = unsafe {
        let cname = CString::new(name).unwrap();
        gl::GetUniformLocation(program, cname.as_ptr())
    };

    unsafe {
        gl::Uniform3fv(location, 1, vec.as_ptr());
    }
}

fn create_position_matrix(position: Vector3<f32>) -> Matrix4<f32> {
    Matrix4::from_translation(position)
}

fn create_rotation_matrix(x: f32, y: f32, z:f32) -> Matrix4<f32> {
    let x = Matrix4::from_angle_x(Rad(x.to_radians()));
    let y = Matrix4::from_angle_y(Rad(y.to_radians()));
    let z = Matrix4::from_angle_z(Rad(z.to_radians()));
    x*y*z
}

fn create_scale_matrix(scale: f32) -> Matrix4<f32> {
    Matrix4::from_scale(scale)
}

fn create_view_matrix(position: Point3<f32>, rotation: Vector3<f32>) -> Matrix4<f32> {
    Matrix4::look_at_rh(
        position, // Posición de la cámara
        Point3::new(0.0 as f32 , 0.0 as f32, 0.0 as f32),
        Vector3::new(0.0, 1.0, 0.0), // El vector "arriba"
    )
}

fn create_projection_matrix(aspect_ratio: f32) -> Matrix4<f32> {
    perspective(Deg(45.0), aspect_ratio, 0.01, 10000.0)
}

fn read_file_to_string(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn parse_obj(file_path: &str) -> Result<Vec<f32>, String> {
    let file = File::open(file_path).map_err(|e| format!("Error al abrir el archivo: {}", e))?;
    let reader = io::BufReader::new(file);

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut textures: Vec<[f32; 2]> = Vec::new();
    let mut faces:Vec<[usize; 3]> = Vec::new();

    // Leer línea por línea
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error al leer una línea: {}", e))?;

        if line.starts_with("v ") {
            // Extraer los vértices
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 4 {
                let x: f32 = parts[1].parse().map_err(|e| format!("Error al parsear x: {}", e))?;
                let y: f32 = parts[2].parse().map_err(|e| format!("Error al parsear y: {}", e))?;
                let z: f32 = parts[3].parse().map_err(|e| format!("Error al parsear z: {}", e))?;
                
                // Agregar las coordenadas en un solo vector
                vertices.push([x, y, z])
            }
        }

        if line.starts_with("vn ") {
            // Extraer los vértices
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 4 {
                let x: f32 = parts[1].parse().map_err(|e| format!("Error al parsear x: {}", e))?;
                let y: f32 = parts[2].parse().map_err(|e| format!("Error al parsear y: {}", e))?;
                let z: f32 = parts[3].parse().map_err(|e| format!("Error al parsear z: {}", e))?;
                
                // Agregar las coordenadas en un solo vector
                normals.push([x, y, z])
            }
        }

        if line.starts_with("vt ") {
            // Extraer los vértices
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let x: f32 = parts[1].parse().map_err(|e| format!("Error al parsear x: {}", e))?;
                let y: f32 = parts[2].parse().map_err(|e| format!("Error al parsear y: {}", e))?;
                
                // Agregar las coordenadas en un solo vector
                textures.push([x, y])
            }
        }

        if line.starts_with("f ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() == 4 {
                // Procesar los tres vértices de la cara
                for i in 1..4 {
                    // Separar cada vértice por "/"
                    let vertex_data: Vec<&str> = parts[i].split('/').collect();
                    
                    // Asegurarnos de que tenemos 3 partes para cada vértice (v/vt/vn)
                    if vertex_data.len() == 3 {
                        let v_idx: usize = vertex_data[0].parse::<usize>().map_err(|e| format!("Error al parsear índice de vértice: {}", e))? - 1;
                        let vt_idx: usize = vertex_data[1].parse::<usize>().map_err(|e| format!("Error al parsear índice de textura: {}", e))? - 1;
                        let vn_idx: usize = vertex_data[2].parse::<usize>().map_err(|e| format!("Error al parsear índice de normal: {}", e))? - 1;
                        
                        // Agregar los índices de los vértices, texturas y normales
                        faces.push([v_idx, vt_idx, vn_idx]);
                    } else {
                        return Err(format!("La cara no tiene el formato correcto: {}", parts[i]));
                    }
                }
            } else {
                return Err(format!("La línea de la cara no tiene el formato esperado: {}", line));
            }
        }
    }

    let mut res = Vec::new();
    for face in faces.iter()
    {
        for e in vertices[face[0]]
        {
            res.push(e);
        }
        
        for e in normals[face[2]]
        {
            res.push(e);
        }
        
        for e in textures[face[1]]
        {
            res.push(e);
        }
    }

    Ok(res)

}