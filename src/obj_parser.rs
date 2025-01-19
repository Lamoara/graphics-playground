use std::{fs::File, io::{self, BufRead}};

enum FaceType
{
    Quad([[usize; 3]; 4]),
    Triangle([[usize; 3]; 3])
}

pub fn parse_obj(file_path: &str) -> Result<Vec<f32>, String> {
    let file = File::open(file_path).map_err(|e| format!("Error al abrir el archivo: {}", e))?;
    let reader = io::BufReader::new(file);

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut textures: Vec<[f32; 2]> = Vec::new();
    let mut faces:Vec<FaceType> = Vec::new();

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
            
            if parts.len() == 4 
            {
                let mut face: [[usize; 3]; 3] = [[0; 3]; 3];
                // Procesar los tres vértices de la cara
                for i in 1..4 
                {
                    // Separar cada vértice por "/"
                    let vertex_data: Vec<&str> = parts[i].split('/').collect();
                    
                    // Asegurarnos de que tenemos 3 partes para cada vértice (v/vt/vn)
                    if vertex_data.len() == 3 
                    {
                        let v_idx: usize = vertex_data[0].parse::<usize>().unwrap_or(1) - 1;
                        let vt_idx: usize = vertex_data[1].parse::<usize>().unwrap_or(1) - 1;
                        let vn_idx: usize = vertex_data[2].parse::<usize>().unwrap_or(1) - 1;
                        
                        // Agregar los índices de los vértices, texturas y normales
                        face[i - 1] = [v_idx, vt_idx, vn_idx];
                    } else 
                    {
                        return Err(format!("La cara no tiene el formato correcto: {}", parts[i]));
                    }
                }
                faces.push(FaceType::Triangle(face));
            } else if parts.len() == 5 
            {
                let mut face: [[usize; 3]; 4] = [[0; 3]; 4];
                // Procesar los tres vértices de la cara
                for i in 1..5 
                {
                    // Separar cada vértice por "/"
                    let vertex_data: Vec<&str> = parts[i].split('/').collect();
                    
                    // Asegurarnos de que tenemos 3 partes para cada vértice (v/vt/vn)
                    if vertex_data.len() == 3 
                    {
                        let v_idx: usize = vertex_data[0].parse::<usize>().unwrap_or(1) - 1;
                        let vt_idx: usize = vertex_data[1].parse::<usize>().unwrap_or(1) - 1;
                        let vn_idx: usize = vertex_data[2].parse::<usize>().unwrap_or(1) - 1;
                        
                        // Agregar los índices de los vértices, texturas y normales
                        face[i - 1] = [v_idx, vt_idx, vn_idx];
                    } else 
                    {
                        return Err(format!("La cara no tiene el formato correcto: {}", parts[i]));
                    }
                }
                faces.push(FaceType::Quad(face));
            }
            else {
                return Err(format!("La línea de la cara no tiene el formato esperado: {}", line));
            }
        }
    }

    let mut corrected_faces: Vec<[usize; 3]> = Vec::new();

    for e in faces
    {
        match e {
            FaceType::Quad(data) => {
                corrected_faces.push(data[0]);
                corrected_faces.push(data[1]);
                corrected_faces.push(data[2]);
                corrected_faces.push(data[0]);
                corrected_faces.push(data[2]);
                corrected_faces.push(data[3]);
            },
            FaceType::Triangle(data) => for e in data{ corrected_faces.push(e)},
        }
    }

    let mut res: Vec<f32> = Vec::new();
    for face in corrected_faces.iter()
    {
        for e in vertices[face[0]]
        {
            res.push(e);
        }
        
        for e in normals[face[2]]
        {
            res.push(e);
        }
        
        if textures.len() == 0{
            res.push(0.0);
            res.push(0.0);
            continue;
        }
        for e in textures[face[1]]
        {
            res.push(e);
        }
    }

    Ok(res)

}