use crate::shape::Shape;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

// TODO use these for shapes
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vector2D<f64>,
    pub rot: f64,
    prev_pos: Vector2D<f64>,
    prev_rot: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct Material {
    restitution: f64,
    density: f64,
    color: Color,
}
pub const ROCK: Material = Material {
    density: 0.6,
    restitution: 0.1,
    color: Color::GRAY,
};
pub const WOOD: Material = Material {
    density: 0.3,
    restitution: 0.2,
    color: Color::RGB(170, 170, 0),
};
pub const METAL: Material = Material {
    density: 1.2,
    restitution: 0.8,
    color: Color::RGB(200, 200, 200),
};
pub const BOUNCY_BALL: Material = Material {
    density: 0.3,
    restitution: 0.8,
    color: Color::CYAN,
};
pub const SUPER_BALL: Material = Material {
    density: 0.3,
    restitution: 0.95,
    color: Color::MAGENTA,
};
pub const PILLOW: Material = Material {
    density: 0.1,
    restitution: 0.2,
    color: Color::WHITE,
};
pub const STATIC: Material = Material {
    density: 0.0,
    restitution: 0.4,
    color: Color::YELLOW,
};

#[derive(Debug)]
pub struct MassData {
    inv_mass: f64,
    inv_inertia: f64,
}

#[derive(Debug)]
pub struct RigidBody {
    shape: Shape,
    pub transform: Transform,
    material: Material,
    mass_data: MassData,
    pub vel: Vector2D<f64>,
    acc: Vector2D<f64>,
}

impl RigidBody {
    pub fn new(pos: Vector2D<f64>, shape: Shape, material: Material) -> Self {
        let mass = material.density * shape.area();
        RigidBody {
            shape,
            transform: Transform {
                pos,
                prev_pos: pos,
                rot: 0.0,
                prev_rot: 0.0,
            },
            material,
            mass_data: MassData {
                inv_mass: if mass == 0.0 { 0.0 } else { mass.recip() },
                inv_inertia: 0.0,
            },
            vel: Vector2D::new(0.0, 0.0),
            acc: Vector2D::new(0.0, 0.0),
        }
    }

    pub fn add_acc(&mut self, acc: Vector2D<f64>) {
        self.acc = acc;
    }
    pub fn add_force(&mut self, force: Vector2D<f64>) {
        self.acc += force * self.mass_data.inv_mass;
    }

    pub fn get_inv_mass(&self) -> f64 {
        self.mass_data.inv_mass
    }
    pub fn get_inv_inertia(&self) -> f64 {
        self.mass_data.inv_inertia
    }
    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }
    pub fn get_restitution(&self) -> f64 {
        self.material.restitution
    }

    pub fn integrate(&mut self, dt: f64) {
        self.transform.prev_pos = self.transform.pos;
        self.vel += self.acc * dt;
        self.transform.pos += self.vel * dt;
        self.acc = Vector2D::new(0.0, 0.0);
    }

    pub fn display(
        &self,
        canvas: &Canvas<Window>,
        interpolation_factor: f64,
    ) -> Result<(), String> {
        self.shape.display(
            canvas,
            &(self.transform.pos * (interpolation_factor)
                + self.transform.prev_pos * (1.0 - interpolation_factor)),
            self.transform.rot * (interpolation_factor)
                + self.transform.prev_rot * (1.0 - interpolation_factor),
            &self.material.color,
        )
    }

    pub fn intersects(&self, other: &RigidBody) -> bool {
        Shape::intersects(
            &self.shape,
            &self.transform.pos,
            &other.shape,
            &other.transform.pos,
        )
    }

    pub fn point_inside(&self, point: &Vector2D<f64>) -> bool {
        self.shape.point_inside(point, &self.transform.pos)
    }
}
