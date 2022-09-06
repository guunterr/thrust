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
    bodies: Vec<RigidBody>,
    selected_index: Option<usize>,
    selected_offset: Option<Vector2D<f64>>,
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
        self.bodies.push(body);
    }

    pub fn update(&mut self, input: &Input) {
        self.handle_user_input(input);
        self.bodies.iter_mut().for_each(|body| body.integrate());

        for i in 0..self.bodies.len() {
            for j in i + 1..self.bodies.len() {
                if !self.bodies[i].intersects(&self.bodies[j]) {
                    continue;
                }
                let collision_data = self.bodies[i].collision_data(&self.bodies[j]);
                println!("\n");
                println!("{:?}", collision_data);
                let rv = self.bodies[j].vel - self.bodies[i].vel;
                println!("rv: {:?}", rv);
                let vel_along_normal = Vector2D::dot(collision_data.normal_vector, rv);
                println!("vel_along_normal: {:?}", vel_along_normal);
                if vel_along_normal > 0.0 {
                    continue;
                }
                let e = self.bodies[i].restitution.min(self.bodies[j].restitution);
                let mut impulse = -(1.0 + e) * vel_along_normal;
                impulse /= 1.0 / self.bodies[i].mass + 1.0 / self.bodies[j].mass;

                let impulse = collision_data.normal_vector * impulse;
                let body_i_mass = self.bodies[i].mass;
                let body_j_mass = self.bodies[j].mass;
                self.bodies[i].vel -= impulse * (1.0 / body_i_mass);
                self.bodies[j].vel += impulse * (1.0 / body_j_mass);
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
                .filter(|(_, body)| body.point_inside(&m))
                .next()
                .map(|element| element.0);

            if let Some(i) = self.selected_index {
                self.selected_offset = Some(m - self.bodies[i].pos)
            }
        }

        if input.is_mouse_down(&MouseButton::Left) {
            if let (Some(i), Some(offset)) = (self.selected_index, self.selected_offset) {
                self.bodies[i].pos = input.mouse_position().as_f64s() - offset;
            }
        }

        if input.is_mouse_released(&MouseButton::Left) {
            if let Some(i) = self.selected_index {
                self.bodies[i].set_vel(Vector2D::new(0.0, 0.0));
            }
        }

        if input.is_mouse_released(&MouseButton::Right) {
            if let (Some(i), Some(offset)) = (self.selected_index, self.selected_offset) {
                let diff = self.bodies[i].pos + offset - input.mouse_position().as_f64s();
                self.bodies[i].set_vel(diff / 10.0);
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
            self.bodies.push(RigidBody::new(
                input.mouse_position().as_f64s(),
                rng.gen_range(1.0..5.0),
                Rect {
                    w: rng.gen_range(20.0..60.0),
                    h: rng.gen_range(20.0..60.0),
                    color: Color::RGB(
                        rng.gen_range(0..=255),
                        rng.gen_range(0..=255),
                        rng.gen_range(0..=255),
                    ),
                },
                rng.gen_range(0.75..0.95),
            ))
        }

        if input.is_key_pressed(&Keycode::X) {
            self.bodies.push(RigidBody::new(
                input.mouse_position().as_f64s(),
                rng.gen_range(1.0..5.0),
                Circle {
                    r: rng.gen_range(10.0..35.0),
                    color: Color::RGB(
                        rng.gen_range(0..=255),
                        rng.gen_range(0..=255),
                        rng.gen_range(0..=255),
                    ),
                },
                rng.gen_range(0.75..0.95),
            ))
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>, input: &Input) {
        self.bodies.iter().for_each(|body| body.display(canvas));

        use sdl2::mouse::MouseButton;
        if input.is_mouse_down(&MouseButton::Right) {
            if let (Some(i), Some(offset)) = (self.selected_index, self.selected_offset) {
                let start = input.mouse_position();
                let end = (self.bodies[i].pos + offset).as_i32s();
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
                    .filter(|(b1, b2)| b1.intersects(b2))
            })
            .map(|(b1, b2)| b1.collision_data(b2))
            .for_each(|manifold| manifold.display(canvas));
    }
}
