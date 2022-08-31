use crate::input_handler::Input;
use crate::rigidbody::RigidBody;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

pub struct PhysicsManager {
    bodies: Vec<RigidBody>,
}
impl PhysicsManager {
    pub fn new() -> PhysicsManager {
        PhysicsManager { bodies: Vec::new() }
    }
    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, input: &Input) {
        use sdl2::mouse::MouseButton;
        if input.is_mouse_down(&MouseButton::Left) {
            let m = input.mouse_position();
            if let Some(body) = self
                .bodies
                .iter_mut()
                .filter(|body| body.shape.point_inside(&m.as_f64s(), &body.pos))
                .next()
            {
                body.pos = m.as_f64s();
            }
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
    pub fn display(&self, canvas: &Canvas<Window>) {
        self.bodies.iter().for_each(|body| body.display(canvas))
    }
}
