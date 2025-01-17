extern crate sdl2;

use graphics_playground::fps_counter::FpsCounter;
use graphics_playground::text::{TextAlignment, TextInstance, TextSettings};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    // Inicializar SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // Crear una ventana
    let window = video_subsystem
        .window("SDL2 en Rust", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut text_instance = TextInstance::new(&ttf_context, "04B_19__.TTF", 14).unwrap();
    text_instance.init(&canvas);
    text_instance.set_position((400, 300));

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut running = true;

    let mut fps_counter_setting = TextSettings::new();
    fps_counter_setting.set_color(Some(Color::GREEN));
    fps_counter_setting.set_scale(2);
    fps_counter_setting.set_position(Some((20, 20)));
    fps_counter_setting.set_alignment(Some(TextAlignment::MidLeft));

    let mut main_text = TextSettings::new();
    main_text.set_color(Some(Color::WHITE));
    main_text.set_scale(1);
    main_text.set_position(Some((400, 300)));
    main_text.set_alignment(Some(TextAlignment::Centered));

    let mut fps_counter = FpsCounter::new(280);

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        fps_counter_setting.load_to_instance(&mut text_instance);
        text_instance.draw_to_canvas(format!("FPS: {}", fps_counter.average_fps()).as_str(), &mut canvas).unwrap();
        main_text.load_to_instance(&mut text_instance);
        text_instance.draw_to_canvas("Esto es simplemente texto general", &mut canvas).unwrap();

        fps_counter.frame(true);
        canvas.present();
    }

    Ok(())
}
