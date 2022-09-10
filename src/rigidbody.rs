use crate::shape::Shape;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;



#[derive(Debug)]
pub struct CollisionData {
    pub collision_point: Vector2D<f64>,
    pub normal_vector: Vector2D<f64>,
    pub depth: f64,
}


impl CollisionData {
    pub fn display(&self, canvas: &Canvas<Window>) {
        let p1 = self.collision_point - self.normal_vector * self.depth / 2.0;
        let p2 = self.collision_point + self.normal_vector * self.depth / 2.0;
        canvas
            .line(
                p1.x as i16,
                p1.y as i16,
                p2.x as i16,
                p2.y as i16,
                Color::RED,
            )
            .unwrap();
        canvas
            .circle(
                self.collision_point.x as i16,
                self.collision_point.y as i16,
                5,
                Color::RED,
            )
            .unwrap();
    }
}

#[derive(Debug, PartialEq)]
pub struct RigidBody {
    pub pos: Vector2D<f64>,
    pub vel: Vector2D<f64>,
    acc: Vector2D<f64>,
    pub mass: f64,
    shape: Shape,
    pub restitution: f64,
}
impl RigidBody {
    pub fn new(pos: Vector2D<f64>, mass: f64, shape: Shape, restitution: f64,) -> Self {
        RigidBody {
            pos,
            vel: Vector2D::new(0.0, 0.0),
            acc: Vector2D::new(0.0, 0.0),
            mass,
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

    pub fn display(&self, canvas: &Canvas<Window>) {
        self.shape.display(canvas, &self.pos)
    }

    pub fn intersects(&self, other: &RigidBody) -> bool {
        Shape::intersects(&self.shape, &self.pos, &other.shape, &other.pos)
    }

    pub fn collision_data(&self, other: &RigidBody) -> Option<CollisionData> {
        Shape::collision_data(&self.shape, &self.pos, &other.shape, &other.pos)
    }

    pub fn point_inside(&self, point: &Vector2D<f64>) -> bool {
        self.shape.point_inside(point, &self.pos)
    }
}
