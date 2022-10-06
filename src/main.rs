extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use thrust::input_handler::Input;
use thrust::physics::PhysicsManager;
use thrust::rigidbody::{RigidBody, BOUNCY_BALL, METAL, STATIC};
use thrust::shape::Shape;

use rand::Rng;
use vector2d::Vector2D;

use std::collections::HashSet;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::env;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 800;
struct Data {
    selected_index: Option<u128>,
    selected_offset: Option<Vector2D<f64>>,
    fixed_objects: HashSet<u128>,
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

    let mut polygons: Vec<Shape> = Vec::new();

    let triangle = Shape::Polygon {
        points: vec![
            Vector2D::new(0.0, 20.0),
            Vector2D::new(40.0, 20.0),
            Vector2D::new(10.0, 90.0),
        ],
    };
    polygons.push(triangle);

    let tri2 = Shape::Polygon {
        points: vec![
            Vector2D::new(40.0, 00.0),
            Vector2D::new(0.0, 50.0),
            Vector2D::new(0.0, 0.0),
        ],
    };
    polygons.push(tri2);

    let diamond = Shape::poly(vec![
        Vector2D::new(0.0, -40.0),
        Vector2D::new(50.0, 0.0),
        Vector2D::new(0.0, 40.0),
        Vector2D::new(-50.0, 0.0),
    ]);
    // Shape::Polygon {
    //     points: vec![
    //         Vector2D::new(0.0, 40.0),
    //         Vector2D::new(-50.0, 0.0),
    //         Vector2D::new(0.0, -40.0),
    //         Vector2D::new(50.0, 0.0),
    //     ],
    // };
    //polygons.push(diamond);

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

        physics_update_time_accumulator += dt;
        while physics_update_time_accumulator > physics_frame_time {
            physics_update_time_accumulator -= physics_frame_time;
            physics_manager.update(physics_frame_time);
        }

        physics_manager.display(
            &mut canvas,
            physics_update_time_accumulator / physics_frame_time,
        )?;

        diamond.display(
            &canvas,
            &input.mouse_position().as_f64s(),
            0.0,
            &Color::RED,
        )?;
        polygons.iter().enumerate().for_each(|(i, poly)| {
            let pos = &Vector2D::new((i + 1) as f64 * 100.0, 200.0);
            let intersects = Shape::intersects(poly, pos, &diamond, &input.mouse_position().as_f64s()); 
            let color = if intersects { Color::MAGENTA } else { Color::CYAN };

            let data = Shape::collision_data(poly, pos, &diamond, &input.mouse_position().as_f64s());

            poly.display(
                &canvas,
                pos,
                0.0,
                &color,
            )
            .unwrap();

            if intersects {
                data.display(&canvas);
            }
        });

        canvas.present();

        let passed_time = Instant::now().duration_since(frame_start_time);
        sleep(Duration::from_secs_f64(1.0 / 60.0).saturating_sub(passed_time));
    }
    Ok(())
}

pub fn add_debug_circle(physics_manager: &mut PhysicsManager, pos: Vector2D<f64>, r: f64) {
    physics_manager.add_body(RigidBody::new(pos, Shape::Circle { r }, BOUNCY_BALL));
}

pub fn add_debug_rect(physics_manager: &mut PhysicsManager, pos: Vector2D<f64>, w: f64, h: f64) {
    physics_manager.add_body(RigidBody::new(pos, Shape::Rect { w, h }, METAL));
}

fn main() -> Result<(), String> {
    env::set_var("RUST_BACKTRACE", "1");
    let init = |physics_manager: &mut PhysicsManager| {
        let wall_thickness: f64 = 1500.0;
        let mut fixed_objects = HashSet::with_capacity(4);
        fixed_objects.insert(physics_manager.add_body(RigidBody::new(
            Vector2D::new(
                SCREEN_WIDTH as f64 / 2.0,
                SCREEN_HEIGHT as f64 + wall_thickness / 2.0,
            ),
            Shape::Rect {
                w: SCREEN_WIDTH as f64 * 10.0,
                h: wall_thickness - 2.0,
            },
            STATIC,
        )));
        fixed_objects.insert(physics_manager.add_body(RigidBody::new(
            Vector2D::new(SCREEN_WIDTH as f64 / 2.0, -wall_thickness / 2.0),
            Shape::Rect {
                w: SCREEN_WIDTH as f64 * 10.0,
                h: wall_thickness - 2.0,
            },
            STATIC,
        )));

        fixed_objects.insert(physics_manager.add_body(RigidBody::new(
            Vector2D::new(0.0 - wall_thickness / 2.0, SCREEN_HEIGHT as f64 / 2.0),
            Shape::Rect {
                w: wall_thickness - 2.0,
                h: SCREEN_HEIGHT as f64 * 10.0,
            },
            STATIC,
        )));

        fixed_objects.insert(physics_manager.add_body(RigidBody::new(
            Vector2D::new(
                SCREEN_WIDTH as f64 + wall_thickness / 2.0,
                SCREEN_HEIGHT as f64 / 2.0,
            ),
            Shape::Rect {
                w: wall_thickness - 2.0,
                h: SCREEN_HEIGHT as f64 * 10.0,
            },
            STATIC,
        )));
        Ok(Data {
            selected_index: None,
            selected_offset: None,
            fixed_objects,
        })
    };
    let update = |data: &mut Data,
                  _dt: f64,
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
                rng.gen_range(35.0..50.0),
                rng.gen_range(35.0..50.0),
            );
        }

        if input.is_key_pressed(&Keycode::X) {
            add_debug_circle(
                physics_manager,
                input.mouse_position().as_f64s(),
                rng.gen_range(25.0..35.0),
            );
        }

        if input.is_key_pressed(&Keycode::C) {
            let bodies = physics_manager.get_body_count();
            for i in 0..bodies {
                if !data.fixed_objects.contains(&i) {
                    physics_manager.delete_body(i)?;
                }
            }
        }

        if input.is_key_pressed(&Keycode::M) {
            add_debug_circle(
                physics_manager,
                input.mouse_position().as_f64s(),
                rng.gen_range(40.0..80.0),
            )
        }
        if input.is_key_pressed(&Keycode::N) {
            add_debug_rect(
                physics_manager,
                input.mouse_position().as_f64s(),
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
