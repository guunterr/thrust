use std::cell::RefCell;

use crate::input_handler::Input;
use crate::rigidbody::RigidBody;
use crate::shape::Shape::{Circle, Rect};
use rand::{self, Rng};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

pub struct PhysicsManager {
    bodies: Vec<RefCell<RigidBody>>,
    selected_index: Option<usize>,
    selected_offset: Option<Vector2D<f64>>,
}

impl Default for PhysicsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsManager {
    pub fn new() -> PhysicsManager {
        PhysicsManager {
            bodies: Vec::new(),
            selected_index: None,
            selected_offset: None,
        }
    }
    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(RefCell::new(body));
    }

    pub fn update(&mut self, input: &Input) {
        self.handle_user_input(input);
        self.bodies.iter_mut().for_each(|body| {
            let mut body = body.borrow_mut();
            if body.inv_mass != 0.0 {
                body.add_acc(Vector2D::new(0.0, 0.5));
            }
            body.integrate(1.0);
        });

        for i in 0..self.bodies.len() {
            for j in i + 1..self.bodies.len() {
                //body i and body j should not be the same so this should never panic
                let mut body_i = self
                    .bodies
                    .get(i)
                    .expect("ith element should exist")
                    .borrow_mut();
                let mut body_j = self
                    .bodies
                    .get(j)
                    .expect("jth object should exist")
                    .borrow_mut();

                let body_i_inv_mass = body_i.inv_mass;
                let body_j_inv_mass = body_j.inv_mass;

                if !body_i.intersects(&body_j) || (body_i_inv_mass == 0.0 && body_j_inv_mass == 0.0)
                {
                    continue;
                }

                let manifold = body_i.manifold(&body_j).unwrap();

                let percent = 0.8;
                let correction = manifold.normal_vector * manifold.depth
                    / (body_i.inv_mass + body_j.inv_mass)
                    * percent;
                body_i.pos -= correction * body_i_inv_mass;
                body_j.pos += correction * body_j_inv_mass;

                let rv = body_j.vel - body_i.vel;
                let vel_along_normal = Vector2D::dot(manifold.normal_vector, rv);
                if vel_along_normal > 0.0 {
                    continue;
                }
                let e = body_i.restitution.min(body_j.restitution);
                let mut impulse = -(1.0 + e) * vel_along_normal;
                impulse /= 1.0 * body_i.inv_mass + 1.0 * body_j.inv_mass;

                let impulse = manifold.normal_vector * impulse;
                body_i.vel -= impulse * body_i_inv_mass;
                body_j.vel += impulse * body_j_inv_mass;
            }
        }
    }

    fn handle_user_input(&mut self, input: &Input) {
        use sdl2::mouse::MouseButton;
        if input.is_mouse_pressed(&MouseButton::Left) || input.is_mouse_pressed(&MouseButton::Right)
        {
            let m = input.mouse_position().as_f64s();
            self.selected_index = self
                .bodies
                .iter_mut()
                .enumerate()
                .find(|(_, body)| body.borrow().point_inside(&m))
                .map(|element| element.0);

            if let Some(i) = self.selected_index {
                self.selected_offset = Some(m - self.bodies[i].borrow().pos)
            }
        }

        if input.is_mouse_down(&MouseButton::Left) {
            if let (Some(i), Some(offset)) = (self.selected_index, self.selected_offset) {
                self.bodies[i].borrow_mut().pos = input.mouse_position().as_f64s() - offset;
            }
        }

        if input.is_mouse_released(&MouseButton::Left) {
            if let Some(i) = self.selected_index {
                self.bodies[i].borrow_mut().set_vel(Vector2D::new(0.0, 0.0));
            }
        }

        if input.is_mouse_released(&MouseButton::Right) {
            if let (Some(i), Some(offset)) = (self.selected_index, self.selected_offset) {
                let diff = self.bodies[i].borrow().pos + offset - input.mouse_position().as_f64s();
                self.bodies[i].borrow_mut().set_vel(diff / 10.0);
            }
        }

        if input.is_mouse_released(&MouseButton::Left)
            || input.is_mouse_released(&MouseButton::Right)
        {
            self.selected_index = None;
            self.selected_offset = None;
        }

        let mut rng = rand::thread_rng();

        if input.is_key_pressed(&Keycode::Z) {
            self.add_debug_rect(
                input.mouse_position().as_f64s(),
                rng.gen_range(1.0..5.0),
                rng.gen_range(40.0..50.0),
                rng.gen_range(40.0..50.0),
            );
        }

        if input.is_key_pressed(&Keycode::X) {
            self.add_debug_circle(
                input.mouse_position().as_f64s(),
                rng.gen_range(1.0..5.0),
                rng.gen_range(20.0..25.0),
            );
        }

        if input.is_key_pressed(&Keycode::C) {
            self.bodies.clear();
        }

        if input.is_key_pressed(&Keycode::M) {
            self.add_debug_circle(
                input.mouse_position().as_f64s(),
                0.0,
                rng.gen_range(40.0..80.0),
            )
        }
        if input.is_key_pressed(&Keycode::N) {
            self.add_debug_rect(
                input.mouse_position().as_f64s(),
                0.0,
                rng.gen_range(40.0..80.0),
                rng.gen_range(40.0..80.0),
            )
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>, input: &Input) {
        self.bodies
            .iter()
            .for_each(|body| body.borrow().display(canvas));

        use sdl2::mouse::MouseButton;
        if input.is_mouse_down(&MouseButton::Right) {
            if let (Some(i), Some(offset)) = (self.selected_index, self.selected_offset) {
                let start = input.mouse_position();
                let end = (self.bodies[i].borrow().pos + offset).as_i32s();
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas
                    .draw_line((start.x, start.y), (end.x, end.y))
                    .unwrap();
            }
        }

        self.bodies
            .iter()
            .enumerate()
            .flat_map(|(i, b1)| {
                self.bodies
                    .iter()
                    .skip(i + 1)
                    .map(move |b2| (b1, b2))
                    .filter(|(b1, b2)| b1.borrow().intersects(&b2.borrow()))
            })
            .map(|(b1, b2)| b1.borrow().manifold(&b2.borrow()))
            .for_each(|manifold| manifold.unwrap().display(canvas));
    }

    pub fn add_debug_circle(&mut self, pos: Vector2D<f64>, mass: f64, r: f64) {
        let mut rng = rand::thread_rng();
        self.add_body(RigidBody::new(
            pos,
            mass,
            Circle {
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
            rng.gen_range(0.85..0.95),
        ));
    }

    pub fn add_debug_rect(&mut self, pos: Vector2D<f64>, mass: f64, w: f64, h: f64) {
        let mut rng = rand::thread_rng();
        self.add_body(RigidBody::new(
            pos,
            mass,
            Rect {
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
            rng.gen_range(0.85..0.95),
        ));
    }
}
