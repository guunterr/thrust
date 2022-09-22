use std::cell::RefCell;
use std::rc::Rc;

use crate::manifold::Manifold;
use crate::rigidbody::RigidBody;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

pub struct PhysicsManager {
    bodies: Vec<Rc<RefCell<RigidBody>>>,
}

impl Default for PhysicsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsManager {
    pub fn new() -> PhysicsManager {
        PhysicsManager { bodies: Vec::new() }
    }
    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(Rc::new(RefCell::new(body)));
    }

    pub fn update(&mut self, dt: f64) {
        self.bodies.iter_mut().for_each(|body| {
            let mut body = body.borrow_mut();
            if body.get_inv_mass() != 0.0 {
                body.add_acc(Vector2D::new(0.0, 9.8) * dt);
            }
            body.integrate(dt);
        });

        for i in 0..self.bodies.len() {
            for j in i + 1..self.bodies.len() {
                //body i and body j should not be the same so this should never panic
                if let Some(mut manifold) = Manifold::new(
                    Rc::clone(self.bodies.get(i).unwrap()),
                    Rc::clone(self.bodies.get(j).unwrap()),
                ) {
                    manifold.resolve();
                }
            }
        }
    }

    pub fn get_body_count(&self) -> usize {
        self.bodies.len()
    }

    pub fn delete_body(&mut self, _i: usize) -> Result<(), String> {
        //To do this, we will need to turn our vec of bodies into a hashset indexed by uuid
        todo!()
    }

    pub fn get_body_at(&self, point: &Vector2D<f64>) -> Option<usize> {
        self.bodies
            .iter()
            .enumerate()
            .find(|(_, body)| body.borrow().point_inside(point))
            .map(|element| element.0)
    }

    pub fn get_body_position(&self, body_index: usize) -> Option<Vector2D<f64>> {
        self.bodies.get(body_index).map(|body| body.borrow().transform.pos)
    }

    pub fn set_body_position(&self, body_index: usize, pos: Vector2D<f64>) -> Result<(), String> {
        if let Some(body) = self.bodies.get(body_index) {
            body.borrow_mut().transform.pos = pos;
            Ok(())
        } else {
            Err("No such body".to_string())
        }
    }
    pub fn set_body_velocity(&self, body_index: usize, vel: Vector2D<f64>) -> Result<(), String> {
        if let Some(body) = self.bodies.get(body_index) {
            body.borrow_mut().vel = vel;
            Ok(())
        } else {
            Err("No such body".to_string())
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>, interpolation_factor: f64) -> Result<(), String> {
        self.bodies
            .iter()
            .try_for_each(|body| body.borrow().display(canvas, interpolation_factor))?;

        // //Debug code to show manifolds
        // for i in 0..self.bodies.len() {
        //     for j in i + 1..self.bodies.len() {
        //         if let Some(manifold) = Manifold::new(
        //             Rc::clone(self.bodies.get(i).unwrap()),
        //             Rc::clone(self.bodies.get(j).unwrap()),
        //         ) {
        //             manifold.display(canvas);
        //         }
        //     }
        // }
        Ok(())
    }

}
