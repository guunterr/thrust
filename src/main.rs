extern crate sdl2;
pub mod input_handler;
pub mod physics;
pub mod rigidbody;
pub mod shape;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use vector2d::Vector2D;

use input_handler::Input;
use physics::PhysicsManager;
use rigidbody::RigidBody;
use shape::Shape;

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
    let mut physics_manager = PhysicsManager::new();

    physics_manager.add_body(RigidBody::new(
        Vector2D::new(300.0, 300.0),
        1.0,
        Shape::Rect {
            w: 50.0,
            h: 50.0,
            color: Color::RGB(0, 255, 0),
        },
    ));
    physics_manager.add_body(RigidBody::new(
        Vector2D::new(100.0, 100.0),
        1.0,
        Shape::Circle {
            r: 50.0,
            color: Color::BLUE,
        },
    ));
    physics_manager.add_body(RigidBody::new(
        Vector2D::new(200.0, 200.0),
        1.0,
        Shape::Rect {
            w: 50.0,
            h: 50.0,
            color: Color::RGB(0, 255, 0),
        },
    ));
    physics_manager.add_body(RigidBody::new(
        Vector2D::new(400.0, 400.0),
        1.0,
        Shape::Circle {
            r: 50.0,
            color: Color::BLUE,
        },
    ));

    let mut event_pump = sdl.event_pump()?;
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

        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();

        physics_manager.update(&input);
        physics_manager.display(&mut canvas, &input);

        canvas.present();

        std::thread::sleep(Duration::from_millis(1_000u64 / 30));
    }

    Ok(())
}
