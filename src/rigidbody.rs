use crate::shape::Shape;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

pub struct RigidBody {
    pub pos: Vector2D<f64>,
    vel: Vector2D<f64>,
    acc: Vector2D<f64>,
    mass: f64,
    shape: Box<dyn Shape>,
}
impl RigidBody {
    pub fn new(pos: Vector2D<f64>, mass: f64, shape: Box<dyn Shape>) -> Self {
        RigidBody {
            pos,
            vel: Vector2D::new(0.0, 0.0),
            acc: Vector2D::new(0.0, 0.0),
            mass,
            shape,
        }
    }
    pub fn set_vel(&mut self, vel: Vector2D<f64>) {
        self.vel = vel;
    }
    pub fn add_acc(&mut self, acc: Vector2D<f64>) {
        self.acc = acc;
    }
    pub fn add_force(&mut self, force: Vector2D<f64>) {
        self.acc += force / self.mass;
    }

    pub fn integrate(&mut self) {
        self.pos += self.vel;
        self.vel += self.acc;
        self.acc = Vector2D::new(0.0, 0.0);

        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
            self.vel.x = self.vel.x.abs();
        }
        if self.pos.x > SCREEN_WIDTH as f64 {
            self.pos.x = SCREEN_WIDTH as f64;
            self.vel.x = -self.vel.x.abs();
        }
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            self.vel.y = self.vel.y.abs();
        }
        if self.pos.y > SCREEN_HEIGHT as f64 {
            self.pos.y = SCREEN_HEIGHT as f64;
            self.vel.y = -self.vel.y.abs();
        }
    }

    pub fn display(&self, canvas: &Canvas<Window>) -> Result<(), String> {
        self.shape.display(canvas, &self.pos)
    }
}
