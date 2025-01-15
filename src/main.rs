extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Inicializar SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Crear una ventana
    let window = video_subsystem
        .window("SDL2 en Rust", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut running = true;

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

        // Dibujar algo
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(100, 100, 200, 200))?;
        canvas.present();

        // Pausar un momento para evitar usar 100% de la CPU
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
