extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    let window = video_subsystem
        .window("Thrust", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl.event_pump()?;
    let mut rect = Rect::new(SCREEN_WIDTH as i32 / 2 - 75, SCREEN_HEIGHT as i32 / 2 - 75, 150, 150);
    let mut velocity = 5;


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.draw_rect(rect)?;
        canvas.fill_rect(rect)?;

        canvas.filled_circle(100, 100, 50, Color::BLUE);
        canvas.present();
        std::thread::sleep(Duration::from_millis(1_000u64 / 30));
        // The rest of the game loop goes here...   
        rect.set_x(rect.x() + velocity);
        if rect.x() < 0 || ((SCREEN_WIDTH - rect.width()) as i32) < rect.x() {
            velocity *= -1;
        }
    }

    Ok(())
}