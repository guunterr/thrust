use crate::manifold::Manifold;
use crate::shape::Shape;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
pub struct RigidBody {
    pub pos: Vector2D<f64>,
    pub vel: Vector2D<f64>,
    acc: Vector2D<f64>,
    pub inv_mass: f64,
    shape: Shape,
    pub restitution: f64,
}
impl RigidBody {
    pub fn new(pos: Vector2D<f64>, mass: f64, shape: Shape, restitution: f64) -> Self {
        RigidBody {
            pos,
            vel: Vector2D::new(0.0, 0.0),
            acc: Vector2D::new(0.0, 0.0),
            inv_mass: if mass == 0.0 { 0.0 } else { 1.0 / mass },
            shape,
            restitution,
        }
    }
    pub fn set_vel(&mut self, vel: Vector2D<f64>) {
        self.vel = vel;
    }
    pub fn add_acc(&mut self, acc: Vector2D<f64>) {
        self.acc = acc;
    }
    pub fn add_force(&mut self, force: Vector2D<f64>) {
        self.acc += force * self.inv_mass;
    }

    pub fn integrate(&mut self, dt: f64) {
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
        self.acc = Vector2D::new(0.0, 0.0);
    }

    pub fn display(&self, canvas: &Canvas<Window>) {
        self.shape.display(canvas, &self.pos)
    }

    pub fn intersects(&self, other: &RigidBody) -> bool {
        Shape::intersects(&self.shape, &self.pos, &other.shape, &other.pos)
    }

    pub fn manifold(&self, other: &RigidBody) -> Option<Manifold> {
        Shape::manifold(&self.shape, &self.pos, &other.shape, &other.pos)
    }

    pub fn point_inside(&self, point: &Vector2D<f64>) -> bool {
        self.shape.point_inside(point, &self.pos)
    }
}
