use std::{cell::RefCell, rc::Rc};

use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};
use vector2d::Vector2D;

use crate::{
    rigidbody::RigidBody,
    shape::{CollisionData, Shape},
};

#[derive(Debug)]
pub struct Manifold {
    pub body1: Rc<RefCell<RigidBody>>,
    pub body2: Rc<RefCell<RigidBody>>,
    pub collision_point: Vector2D<f64>,
    pub normal_vector: Vector2D<f64>,
    pub depth: f64,
}
impl Manifold {
    pub fn new(body1: Rc<RefCell<RigidBody>>, body2: Rc<RefCell<RigidBody>>) -> Self {
        let CollisionData {
            collision_point,
            normal_vector,
            depth,
        } = Shape::collision_data(
            body1.borrow().get_shape(),
            &body1.borrow().transform.pos,
            body2.borrow().get_shape(),
            &body2.borrow().transform.pos,
        );

        Manifold {
            body1,
            body2,
            collision_point,
            normal_vector,
            depth,
        }
    }

    fn depth_correct(&self, body_i: &mut RigidBody, body_j: &mut RigidBody) {
        let body_i_inv_mass = body_i.get_inv_mass();
        let body_j_inv_mass = body_j.get_inv_mass();

        let percent = 0.5;
        let correction = self.normal_vector * self.depth
            / (body_i.get_inv_mass() + body_j.get_inv_mass())
            * percent;
        body_i.transform.pos -= correction * body_i_inv_mass;
        body_j.transform.pos += correction * body_j_inv_mass;
    }

    fn resolve_impulse(&self, body_i: &mut RigidBody, body_j: &mut RigidBody) {
        let body_i_inv_mass = body_i.get_inv_mass();
        let body_j_inv_mass = body_j.get_inv_mass();

        let rv = body_j.vel - body_i.vel;
        let vel_along_normal = Vector2D::dot(self.normal_vector, rv);
        if vel_along_normal > 0.0 {
            return;
        }
        let e = body_i.get_restitution().min(body_j.get_restitution());
        let mut impulse = -(1.0 + e) * vel_along_normal;
        impulse /= 1.0 * body_i.get_inv_mass() + 1.0 * body_j.get_inv_mass();

        let impulse = self.normal_vector * impulse;
        body_i.vel -= impulse * body_i_inv_mass;
        body_j.vel += impulse * body_j_inv_mass;
    }

    pub fn resolve(&mut self) {
        let mut body_i = self.body1.borrow_mut();
        let mut body_j = self.body2.borrow_mut();

        if !body_i.intersects(&body_i) {
            return;
        }

        if body_i.get_inv_mass() == 0.0 && body_j.get_inv_mass() == 0.0 {
            return;
        }

        self.depth_correct(&mut body_i, &mut body_j);
        self.resolve_impulse(&mut body_i, &mut body_j);
    }

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
