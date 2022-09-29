use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::manifold::Manifold;
use crate::rigidbody::RigidBody;
use crate::shape::Shape;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

type BodyPair = (Rc<RefCell<RigidBody>>, Rc<RefCell<RigidBody>>);

pub struct PhysicsManager {
    bodies: HashMap<u128, Rc<RefCell<RigidBody>>>,
    index: u128,
}

impl Default for PhysicsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsManager {
    pub fn new() -> PhysicsManager {
        PhysicsManager {
            bodies: HashMap::new(),
            index: 0,
        }
    }
    pub fn add_body(&mut self, body: RigidBody) -> u128 {
        self.bodies.insert(self.index, Rc::new(RefCell::new(body)));
        self.index += 1;
        self.index - 1
    }

    pub fn update(&mut self, dt: f64) {
        self.bodies.iter_mut().for_each(|(_index, body)| {
            let mut body = body.borrow_mut();
            if body.get_inv_mass() != 0.0 {
                body.add_acc(Vector2D::new(0.0, 3500.0));
            }
            body.integrate(dt);
        });

        let broad_phase_pairs = &Self::broad_phase(
            &self
                .bodies
                .values()
                .into_iter()
                .cloned()
                .collect::<Vec<Rc<RefCell<RigidBody>>>>(),
        );
        let colliding_pairs = Self::narrow_phase(broad_phase_pairs);
        assert!(colliding_pairs
            .iter()
            .all(|(a, b)| a.borrow().intersects(&b.borrow())));

        colliding_pairs.iter().for_each(|(a, b)| {
            //body i and body j should not be the same so this should be ok
            Manifold::new(Rc::clone(a), Rc::clone(b)).resolve();
        });
    }

    fn broad_phase(bodies: &[Rc<RefCell<RigidBody>>]) -> Vec<BodyPair> {
        let mut out = Vec::with_capacity(bodies.len());
        for i in 0..bodies.len() {
            let body_i = bodies[i].borrow();
            for j in i + 1..bodies.len() {
                let body_j = bodies[j].borrow();
                if Shape::intersects(
                    &body_i.get_shape().get_aabb(),
                    &body_i.transform.pos,
                    &body_j.get_shape().get_aabb(),
                    &body_j.transform.pos,
                ) {
                    out.push((bodies[i].clone(), bodies[j].clone()))
                }
            }
        }
        out
    }

    fn narrow_phase(body_pairs: &[BodyPair]) -> Vec<&BodyPair> {
        body_pairs
            .iter()
            .filter(|(a, b)| (*a).borrow().intersects(&*b.borrow()))
            .collect()
    }

    pub fn get_body_count(&self) -> u128 {
        self.bodies.len() as u128
    }

    pub fn delete_body(&mut self, i: u128) -> Result<(), String> {
        self.bodies
            .remove(&i)
            .ok_or_else(|| "No body with that id".to_string())
            .map(|_| ())
    }

    pub fn get_body_at(&self, point: &Vector2D<f64>) -> Option<u128> {
        self.bodies
            .iter()
            .find(|(_index, body)| body.borrow().point_inside(point))
            .map(|element| *element.0)
    }

    pub fn get_body_position(&self, body_index: u128) -> Option<Vector2D<f64>> {
        self.bodies
            .get(&body_index)
            .map(|body| body.borrow().transform.pos)
    }

    pub fn set_body_position(&self, body_index: u128, pos: Vector2D<f64>) -> Result<(), String> {
        if let Some(body) = self.bodies.get(&body_index) {
            body.borrow_mut().transform.pos = pos;
            Ok(())
        } else {
            Err("No such body".to_string())
        }
    }
    pub fn set_body_velocity(&self, body_index: u128, vel: Vector2D<f64>) -> Result<(), String> {
        if let Some(body) = self.bodies.get(&body_index) {
            body.borrow_mut().vel = vel;
            Ok(())
        } else {
            Err("No such body".to_string())
        }
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        interpolation_factor: f64,
    ) -> Result<(), String> {
        self.bodies
            .iter()
            .try_for_each(|(_i, body)| body.borrow().display(canvas, interpolation_factor))?;

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
