use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};
use vector2d::Vector2D;

#[derive(Debug)]
pub struct Manifold {
    pub collision_point: Vector2D<f64>,
    pub normal_vector: Vector2D<f64>,
    pub depth: f64,
}

impl Manifold {
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
