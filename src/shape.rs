use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
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
                            (pos.x - w / 2.0) as i16,
                            (pos.x - w / 2.0) as i16,
                            (pos.x + w / 2.0) as i16,
                            (pos.x + w / 2.0) as i16,
                        ],
                        &[
                            (pos.y - h / 2.0) as i16,
                            (pos.y + h / 2.0) as i16,
                            (pos.y + h / 2.0) as i16,
                            (pos.y - h / 2.0) as i16,
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
                point.x > x - w / 2.0
                    && point.x < x + w / 2.0
                    && point.y > y - h / 2.0
                    && point.y < y + h / 2.0
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
            (Shape::Rect { w: w1, h: h1, .. }, Shape::Rect { w: w2, h: h2 ,..}) => {
                let diff = pos1 - pos2;
                diff.x.abs() <= (w1 + w2)/2.0 && diff.y.abs() <= (h1 + h2)/2.0
            }
            (Shape::Rect { w, h, .. }, Shape::Circle { r, .. }) => {
                let close_x = pos2.x.max(pos1.x - w / 2.0).min(pos1.x + w / 2.0);
                let close_y = pos2.y.max(pos1.y - h / 2.0).min(pos1.y + h / 2.0);
                let dist = (pos2 - &Vector2D::new(close_x, close_y)).length_squared();
                dist <= r.powi(2)
            }
            (Shape::Circle { r: r1, .. }, Shape::Circle { r: r2, .. }) => {
                let dist = (pos1 - pos2).length_squared();
                dist <= (r1 + r2).powi(2)
            }
            (shape1, shape2) => Shape::intersects(shape2, pos2, shape1, pos1),
        }
    }

    pub fn collision_data(
        shape1: &Shape,
        pos1: &Vector2D<f64>,
        shape2: &Shape,
        pos2: &Vector2D<f64>,
    ) -> (Vector2D<f64>, f64) {
        match (shape1, shape2) {
            (Shape::Rect { w: w1, h: h1, .. }, Shape::Rect { w: w2, h: h2,.. }) => {
                let displacement_vector = pos1 - pos2;
                let depth_vector = displacement_vector - Vector2D::new(w1+w2, h1+h2);
                if depth_vector.x > depth_vector.y {
                    (displacement_vector.horizontal(), 0.0)
                } else {
                    (displacement_vector.vertical(), 0.0)
                }
            }
            (Shape::Rect { w, h, .. }, Shape::Circle { r, .. }) => {
                let close_x = pos2.x.max(pos1.x - w / 2.0).min(pos1.x + w / 2.0);
                let close_y = pos2.y.max(pos1.y - h / 2.0).min(pos1.y + h / 2.0);
                let close = &Vector2D::new(close_x, close_y);
                let dist = (pos2 - close).length_squared();
                ((pos2 - close).normalise(), dist - r)
            }
            (Shape::Circle { r: r1, .. }, Shape::Circle { r: r2, .. }) => {
                let diff = pos2 - pos1;
                (diff.normalise(), diff.length() - (r1 + r2))
            }
            (shape1, shape2) => {
                let (normal_vector, depth) = Shape::collision_data(shape2, pos2, shape1, pos1);
                (normal_vector * -1.0, depth)
            }
        }
    }
}
