extern crate sdl2;
pub mod input_handler;
pub mod manifold;
pub mod physics;
pub mod rigidbody;
pub mod shape;

use rand::Rng;
use rigidbody::{RigidBody, ROCK, STATIC};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use shape::Shape;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use vector2d::Vector2D;

use input_handler::Input;
use physics::PhysicsManager;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 800;
struct Data {
    selected_index: Option<usize>,
    selected_offset: Option<Vector2D<f64>>,
}

fn start_game<T, F1, F2>(init: F1, update: F2) -> Result<(), String>
where
    F1: FnOnce(&mut PhysicsManager) -> Result<T, String>,
    F2: Fn(&mut T, f64, &mut Canvas<Window>, &Input, &mut PhysicsManager) -> Result<(), String>,
{
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

    let mut physics_update_time_accumulator = 0.0;
    let physics_frame_time = 1.0 / 120.0;

    let mut frame_start_time = Instant::now();
    let mut data = init(&mut physics_manager)?;

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
        let dt = Instant::now()
            .duration_since(frame_start_time)
            .as_secs_f64();

        frame_start_time = Instant::now();

        update(&mut data, dt, &mut canvas, &mut input, &mut physics_manager)?;

        if input.is_key_pressed(&Keycode::Space) {
            sleep(Duration::from_millis(1000));
        }

        physics_update_time_accumulator += dt;
        while physics_update_time_accumulator > physics_frame_time {
            physics_update_time_accumulator -= physics_frame_time;
            physics_manager.update(physics_frame_time);
        }

        physics_manager.display(
            &mut canvas,
            physics_update_time_accumulator / physics_frame_time,
        )?;
        canvas.present();

        let passed_time = Instant::now().duration_since(frame_start_time);
        sleep(Duration::from_secs_f64(1.0 / 60.0).saturating_sub(passed_time));
    }
    Ok(())
}

pub fn add_debug_circle(
    physics_manager: &mut PhysicsManager,
    pos: Vector2D<f64>,
    mass: f64,
    r: f64,
) {
    let mut rng = rand::thread_rng();
    physics_manager.add_body(RigidBody::new(
        pos,
        Shape::Circle {
            r,
            color: if mass == 0.0 {
                Color::RGB(255, 255, 255)
            } else {
                Color::RGB(
                    rng.gen_range(0..=255),
                    rng.gen_range(0..=255),
                    rng.gen_range(0..=255),
                )
            },
        },
        ROCK,
    ));
}

pub fn add_debug_rect(
    physics_manager: &mut PhysicsManager,
    pos: Vector2D<f64>,
    mass: f64,
    w: f64,
    h: f64,
) {
    let mut rng = rand::thread_rng();
    physics_manager.add_body(RigidBody::new(
        pos,
        Shape::Rect {
            w,
            h,
            color: if mass == 0.0 {
                Color::RGB(255, 255, 255)
            } else {
                Color::RGB(
                    rng.gen_range(0..=255),
                    rng.gen_range(0..=255),
                    rng.gen_range(0..=255),
                )
            },
        },
        ROCK,
    ));
}

fn main() -> Result<(), String> {
    let init = |physics_manager: &mut PhysicsManager| {
        let wall_thickness: f64 = 1500.0;
        physics_manager.add_body(RigidBody::new(
            Vector2D::new(
                SCREEN_WIDTH as f64 / 2.0,
                SCREEN_HEIGHT as f64 + wall_thickness / 2.0,
            ),
            Shape::Rect {
                w: SCREEN_WIDTH as f64 * 10.0,
                h: wall_thickness - 2.0,
                color: Color::WHITE,
            },
            STATIC,
        ));
        physics_manager.add_body(RigidBody::new(
            Vector2D::new(SCREEN_WIDTH as f64 / 2.0, -wall_thickness / 2.0),
            Shape::Rect {
                w: SCREEN_WIDTH as f64 * 10.0,
                h: wall_thickness - 2.0,
                color: Color::WHITE,
            },
            STATIC,
        ));

        physics_manager.add_body(RigidBody::new(
            Vector2D::new(0.0 - wall_thickness / 2.0, SCREEN_HEIGHT as f64 / 2.0),
            Shape::Rect {
                w: wall_thickness - 2.0,
                h: SCREEN_HEIGHT as f64 * 10.0,
                color: Color::WHITE,
            },
            STATIC,
        ));

        physics_manager.add_body(RigidBody::new(
            Vector2D::new(
                SCREEN_WIDTH as f64 + wall_thickness / 2.0,
                SCREEN_HEIGHT as f64 / 2.0,
            ),
            Shape::Rect {
                w: wall_thickness - 2.0,
                h: SCREEN_HEIGHT as f64 * 10.0,
                color: Color::WHITE,
            },
            STATIC,
        ));
        Ok(Data {
            selected_index: None,
            selected_offset: None,
        })
    };
    let update = |data: &mut Data,
                  dt: f64,
                  canvas: &mut Canvas<Window>,
                  input: &Input,
                  physics_manager: &mut PhysicsManager| {
        if input.is_mouse_pressed(&MouseButton::Left) || input.is_mouse_pressed(&MouseButton::Right)
        {
            let m = input.mouse_position().as_f64s();
            data.selected_index = physics_manager.get_body_at(&m);

            if let Some(i) = data.selected_index {
                data.selected_offset = Some(m - physics_manager.get_body_position(i).unwrap());
            }
        }

        if input.is_mouse_down(&MouseButton::Left) {
            if let (Some(i), Some(offset)) = (data.selected_index, data.selected_offset) {
                physics_manager.set_body_position(i, input.mouse_position().as_f64s() - offset)?;
            }
        }

        if input.is_mouse_released(&MouseButton::Left) {
            if let Some(i) = data.selected_index {
                physics_manager.set_body_velocity(i, Vector2D::new(0.0, 0.0))?;
            }
        }

        if input.is_mouse_released(&MouseButton::Right) {
            if let (Some(i), Some(offset)) = (data.selected_index, data.selected_offset) {
                let pos = physics_manager.get_body_position(i).unwrap();
                let diff = pos + offset - input.mouse_position().as_f64s();
                physics_manager.set_body_velocity(i, diff * 20.0)?;
            }
        }

        if input.is_mouse_released(&MouseButton::Left)
            || input.is_mouse_released(&MouseButton::Right)
        {
            data.selected_index = None;
            data.selected_offset = None;
        }

        let mut rng = rand::thread_rng();

        if input.is_key_pressed(&Keycode::Z) {
            add_debug_rect(
                physics_manager,
                input.mouse_position().as_f64s(),
                rng.gen_range(1.0..5.0),
                rng.gen_range(18.0..27.0),
                rng.gen_range(18.0..27.0),
            );
        }

        if input.is_key_pressed(&Keycode::X) {
            add_debug_circle(
                physics_manager,
                input.mouse_position().as_f64s(),
                rng.gen_range(1.0..5.0),
                rng.gen_range(12.0..20.0),
            );
        }

        if input.is_key_pressed(&Keycode::C) {
            let bodies = physics_manager.get_body_count();
            for i in 0..bodies {
                physics_manager.delete_body(i).unwrap();
            }
        }

        if input.is_key_pressed(&Keycode::M) {
            add_debug_circle(
                physics_manager,
                input.mouse_position().as_f64s(),
                0.0,
                rng.gen_range(40.0..80.0),
            )
        }
        if input.is_key_pressed(&Keycode::N) {
            add_debug_rect(
                physics_manager,
                input.mouse_position().as_f64s(),
                0.0,
                rng.gen_range(40.0..80.0),
                rng.gen_range(40.0..80.0),
            )
        }

        if input.is_mouse_down(&MouseButton::Right) {
            if let (Some(i), Some(offset)) = (data.selected_index, data.selected_offset) {
                let start = input.mouse_position();
                let end = (physics_manager.get_body_position(i).unwrap() + offset).as_i32s();
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.draw_line((start.x, start.y), (end.x, end.y))?;
            }
        }
        Ok(())
    };
    start_game(init, update)?;

    Ok(())
}
