use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

pub enum Shape {
    Rect { w: f64, h: f64, color: Color },
    Circle { r: f64, color: Color },
}


impl Shape {
    pub fn display(&self, canvas: &Canvas<Window>, pos: &Vector2D<f64>) {
        match self {
            Shape::Rect { w, h, color } => {
                canvas
                    .filled_polygon(
                        &[
                            (pos.x - w/2.0) as i16,
                            (pos.x - w/2.0) as i16,
                            (pos.x + w/2.0) as i16,
                            (pos.x + w/2.0) as i16,
                        ],
                        &[
                            (pos.y - h/2.0) as i16,
                            (pos.y + h/2.0) as i16,
                            (pos.y + h/2.0) as i16,
                            (pos.y - h/2.0) as i16,
                        ],
                        *color,
                    )
                    .unwrap();
            }
            Shape::Circle { r, color } => {
                canvas
                    .filled_circle(pos.x as i16, pos.y as i16, *r as i16, *color)
                    .unwrap();
            }
        }
    }

    pub fn point_inside(&self, offset: &Vector2D<f64>, point: &Vector2D<f64>) -> bool {
        return match self {
            Shape::Rect { w, h, .. } => {
                let &Vector2D { x, y } = offset;
                point.x > x-w/2.0 && point.x < x+w/2.0 && point.y > y-h/2.0 && point.y < y+h/2.0
            }
            Shape::Circle { r, .. } => {
                let dist = (offset - point).length_squared();
                dist < r.powi(2)
            }
        };
    }

    pub fn intersects(
        shape1: &Shape,
        pos1: &Vector2D<f64>,
        shape2: &Shape,
        pos2: &Vector2D<f64>,
    ) -> bool {
        match (shape1, shape2) {
            (Shape::Rect { w: w1, h: h1, .. }, Shape::Rect { .. }) => {
                pos2.x > pos1.x && pos2.x < pos1.x + w1 && pos2.y > pos1.y && pos2.y < pos1.y + h1
            },
            (Shape::Rect { w, h, .. }, Shape::Circle { r, .. }) => {
                let close_x = pos2.x.max(pos1.x-w/2.0).min(pos1.x+w/2.0);
                let close_y = pos2.y.max(pos1.y-h/2.0).min(pos1.y+h/2.0);
                let dist = (pos2 - &Vector2D::new(close_x, close_y)).length_squared();
                dist < r.powi(2)
            },
            (Shape::Circle { r: r1, .. }, Shape::Circle { r: r2, .. }) => {
                let dist = (*pos1 - *pos2).length_squared();
                dist < (r1 + r2).powi(2)
            }
            (shape1, shape2) => Shape::intersects(shape2, pos2, shape1, pos1),
        }
    }
}
