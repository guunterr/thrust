use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;
pub trait Shape {
    fn display(&self, canvas: &Canvas<Window>) -> Result<(), String>;
    fn collides(&self, other: &dyn Shape) -> bool;
}

pub struct Circle {
    pub pos: Vector2D<f64>,
    pub r: f64,
    pub color: Color,
}
impl Circle {
    pub fn new(pos: Vector2D<f64>, r: f64, color: Color) -> Self {
        Circle { pos, r, color }
    }
}

impl Shape for Circle {
    fn display(&self, canvas: &Canvas<Window>) -> Result<(), String> {
        canvas.filled_circle(
            self.pos.x as i16,
            self.pos.y as i16,
            self.r as i16,
            self.color,
        )
    }
    fn collides(&self, other: &dyn Shape) -> bool {
        false
    }
}

pub struct Rect {
    pub pos: Vector2D<f64>,
    pub w: f64,
    pub h: f64,
    pub color: Color,
}
impl Rect {
    pub fn new(pos: Vector2D<f64>, w: f64, h: f64, color: Color) -> Self {
        Rect { pos, w, h, color }
    }
}
impl Shape for Rect {
    fn display(&self, canvas: &Canvas<Window>) -> Result<(), String> {
        canvas.filled_polygon(
            &[
                self.pos.x as i16,
                self.pos.x as i16,
                (self.pos.x + self.w) as i16,
                (self.pos.x + self.w) as i16,
            ],
            &[
                self.pos.y as i16,
                (self.pos.y + self.h) as i16,
                (self.pos.y + self.h) as i16,
                self.pos.y as i16,
            ],
            self.color,
        )
    }
    fn collides(&self, other: &dyn Shape) -> bool {
        false
    }
}
