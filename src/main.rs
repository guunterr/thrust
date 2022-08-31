extern crate sdl2;
pub mod input_handler;
pub mod rigidbody;
pub mod shape;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use vector2d::Vector2D;

use input_handler::Input;
use rigidbody::RigidBody;
use shape::{Circle, Rect};

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

    let shape_acc = 3.0;
    let mut rect = RigidBody::new(
        Vector2D::new(300.0, 300.0),
        1.0,
        Box::new(Rect::new(
            Vector2D::new(-25.0, -25.0),
            50.0,
            50.0,
            Color::RGB(0, 255, 0),
        )),
    );

    let mut circle = RigidBody::new(
        Vector2D::new(100.0, 100.0),
        1.0,
        Box::new(Circle::new(Vector2D::new(0.0, 0.0), 50.0, Color::BLUE)),
    );

    'running: loop {
        input.update();
        for event in event_pump.poll_iter() {
            input.handle_event(&event);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if input.is_key_down(&Keycode::Left) {
            rect.add_force(Vector2D::new(-shape_acc, 0.0));
        }
        if input.is_key_down(&Keycode::Right) {
            rect.add_force(Vector2D::new(shape_acc, 0.0));
        }
        if input.is_key_down(&Keycode::Up) {
            rect.add_force(Vector2D::new(0.0, -shape_acc));
        }
        if input.is_key_down(&Keycode::Down) {
            rect.add_force(Vector2D::new(0.0, shape_acc));
        }

        if input.is_key_down(&Keycode::A) {
            circle.add_force(Vector2D::new(-shape_acc, 0.0));
        }
        if input.is_key_down(&Keycode::D) {
            circle.add_force(Vector2D::new(shape_acc, 0.0));
        }
        if input.is_key_down(&Keycode::W) {
            circle.add_force(Vector2D::new(0.0, -shape_acc));
        }
        if input.is_key_down(&Keycode::S) {
            circle.add_force(Vector2D::new(0.0, shape_acc));
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        circle.integrate();
        rect.integrate();
        rect.display(&canvas)?;
        circle.display(&canvas)?;

        canvas.present();

        std::thread::sleep(Duration::from_millis(1_000u64 / 30));
    }

    Ok(())
}
