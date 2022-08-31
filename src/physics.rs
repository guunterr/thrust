use crate::input_handler::Input;
use crate::rigidbody::RigidBody;
use sdl2::keyboard::Keycode;
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
        use sdl2::mouse::MouseButton;
        if input.is_mouse_pressed(&MouseButton::Left) || input.is_mouse_pressed(&MouseButton::Right)
        {
            let m = input.mouse_position().as_f64s();
            self.selected_index = self
                .bodies
                .iter_mut()
                .enumerate()
                .filter(|(_, body)| body.shape.point_inside(&m, &body.pos))
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

        let shape_acc = 3.0;
        if input.is_key_down(&Keycode::Left) {
            self.bodies[0].add_force(Vector2D::new(-shape_acc, 0.0));
        }
        if input.is_key_down(&Keycode::Right) {
            self.bodies[0].add_force(Vector2D::new(shape_acc, 0.0));
        }
        if input.is_key_down(&Keycode::Up) {
            self.bodies[0].add_force(Vector2D::new(0.0, -shape_acc));
        }
        if input.is_key_down(&Keycode::Down) {
            self.bodies[0].add_force(Vector2D::new(0.0, shape_acc));
        }

        if input.is_key_down(&Keycode::A) {
            self.bodies[1].add_force(Vector2D::new(-shape_acc, 0.0));
        }
        if input.is_key_down(&Keycode::D) {
            self.bodies[1].add_force(Vector2D::new(shape_acc, 0.0));
        }
        if input.is_key_down(&Keycode::W) {
            self.bodies[1].add_force(Vector2D::new(0.0, -shape_acc));
        }
        if input.is_key_down(&Keycode::S) {
            self.bodies[1].add_force(Vector2D::new(0.0, shape_acc));
        }

        self.bodies.iter_mut().for_each(|body| body.integrate());
    }

    pub fn display(&self, canvas: &mut Canvas<Window>, input: &Input) {
        self.bodies.iter().for_each(|body| body.display(canvas));

        use sdl2::mouse::MouseButton;
        use sdl2::pixels::Color;
        if input.is_mouse_down(&MouseButton::Right) {
            if let (Some(i), Some(offset)) = (self.selected_index, self.selected_offset) {
                let start = input.mouse_position();
                let end = (self.bodies[i].pos + offset).as_i32s();
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.draw_line((start.x, start.y), (end.x, end.y)).unwrap();
            }
        }
    }
}
