extern crate sdl2;
pub mod input_handler;
pub mod shape;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use vector2d::Vector2D;

use input_handler::Input;
use shape::{Circle, Rect, Shape};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
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
    let mut input = Input::new();

    let mut event_pump = sdl.event_pump()?;

    let mut rect = Rect::new(
        Vector2D::new(300.0, 300.0),
        50.0,
        50.0,
        Color::RGB(0, 255, 0),
    );
    let mut velocity = Vector2D::new(5.0, 0.0);
    let mut circle = Circle::new(Vector2D::new(100.0, 100.0), 50.0, Color::BLUE);

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

        input.get_keyboard_state(&event_pump);

        if input.is_key_down(&Keycode::Left) {
            circle.pos.x -= 5.0;
        }
        if input.is_key_down(&Keycode::Right) {
            circle.pos.x += 5.0;
        }
        if input.is_key_down(&Keycode::Up) {
            circle.pos.y -= 5.0;
        }
        if input.is_key_down(&Keycode::Down) {
            circle.pos.y += 5.0;
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        rect.display(&canvas)?;
        circle.display(&canvas)?;

        canvas.present();

        std::thread::sleep(Duration::from_millis(1_000u64 / 30));

        rect.pos += velocity;
        if rect.pos.x < 0.0 || ((SCREEN_WIDTH - rect.w as u32) as i32) < rect.pos.x as i32 {
            velocity *= -1.0;
        }
    }

    Ok(())
}
