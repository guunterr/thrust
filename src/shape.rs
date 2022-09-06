use crate::rigidbody::CollisionData;
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
            (Shape::Rect { w: w1, h: h1, .. }, Shape::Rect { w: w2, h: h2, .. }) => {
                let diff = pos1 - pos2;
                diff.x.abs() <= (w1 + w2) / 2.0 && diff.y.abs() <= (h1 + h2) / 2.0
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
    ) -> CollisionData {
        match (shape1, shape2) {
            (Shape::Rect { w: w1, h: h1, .. }, Shape::Rect { w: w2, h: h2, .. }) => {
                let lower_corner_x = f64::max(pos1.x - w1 / 2.0, pos2.x - w2 / 2.0);
                let lower_corner_y = f64::max(pos1.y - h1 / 2.0, pos2.y - h2 / 2.0);
                let upper_corner_x = f64::min(pos1.x + w1 / 2.0, pos2.x + w2 / 2.0);
                let upper_corner_y = f64::min(pos1.y + h1 / 2.0, pos2.y + h2 / 2.0);
                let collision_point = Vector2D::new(
                    (lower_corner_x + upper_corner_x) / 2.0,
                    (lower_corner_y + upper_corner_y) / 2.0,
                );
                if upper_corner_x - lower_corner_x > upper_corner_y - lower_corner_y {
                    CollisionData {
                        collision_point,
                        normal_vector: Vector2D::new(0.0, if pos1.y < pos2.y { 1.0 } else { -1.0 }),
                        depth: upper_corner_x - lower_corner_x,
                    }
                } else {
                    CollisionData {
                        collision_point,
                        normal_vector: Vector2D::new(if pos1.y < pos2.y { 1.0 } else { -1.0 }, 0.0),
                        depth: upper_corner_y - lower_corner_y,
                    }
                }
            }
            (Shape::Rect { w, h, .. }, Shape::Circle { r, .. }) => {
                //FIXME This entire function is some major whack
                let close_x = pos2.x.max(pos1.x - w / 2.0).min(pos1.x + w / 2.0);
                let close_y = pos2.y.max(pos1.y - h / 2.0).min(pos1.y + h / 2.0);
                let close = &Vector2D::new(close_x, close_y);
                let dist = (pos2 - close).length();
                CollisionData {
                    collision_point: (close + pos2) / 2.0, //FIXME https://stackoverflow.com/questions/401847/circle-rectangle-collision-detection-intersection
                    normal_vector: if close == pos2 {
                        (pos2 - pos1).normalise()
                    } else {
                        (pos2 - close).normalise()
                    },
                    depth: dist - r,
                }
            }
            (Shape::Circle { r: r1, .. }, Shape::Circle { r: r2, .. }) => {
                let diff = pos2 - pos1;
                let overlap = diff.length() - (r1 + r2);
                let norm = diff.normalise();
                CollisionData {
                    collision_point: pos1 + &(norm * (overlap / 2.0 + r1)),
                    normal_vector: norm,
                    depth: overlap,
                }
            }
            (shape1, shape2) => {
                let mut collision_data = Shape::collision_data(shape2, pos2, shape1, pos1);
                collision_data.normal_vector *= -1.0;
                collision_data
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sdl2::pixels::Color;
    use vector2d::Vector2D;

    use super::Shape::{self, Circle, Rect};
    #[test]
    fn rectangle_intersection_test() {
        let shape1 = &Rect {
            w: 20.0,
            h: 30.0,
            color: Color::RGB(255, 255, 255),
        };
        let shape2 = &Rect {
            w: 50.0,
            h: 10.0,
            color: Color::RGB(255, 0, 255),
        };
        let pos1 = &Vector2D::new(100.0, 100.0);
        let pos2 = &Vector2D::new(130.0, 110.0);

        assert!(
            Shape::intersects(shape1, pos1, shape2, pos2),
            "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
            shape1,
            pos1,
            shape2,
            pos2
        );

        let pos1 = &Vector2D::new(100.0, 100.0);
        let pos2 = &Vector2D::new(135.0, 115.0);

        assert!(
            Shape::intersects(shape1, pos1, shape2, pos2),
            "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
            shape1,
            pos1,
            shape2,
            pos2
        );

        let pos1 = &Vector2D::new(100.0, 100.0);
        let pos2 = &Vector2D::new(150.0, 130.0);

        assert!(
            !Shape::intersects(shape1, pos1, shape2, pos2),
            "Test Failed! {:?} at {:?} should not intersect {:?} at {:?}",
            shape1,
            pos1,
            shape2,
            pos2
        );
    }

    #[test]
    fn circle_intersection_test() {
        let shape1 = &Circle {
            r: 30.0,
            color: Color::RGB(255, 255, 255),
        };
        let shape2 = &Circle {
            r: 20.0,
            color: Color::RGB(255, 255, 255),
        };

        let pos1 = &Vector2D::new(100.0, 100.0);
        let pos2 = &Vector2D::new(130.0, 100.0);

        assert!(
            Shape::intersects(shape1, pos1, shape2, pos2),
            "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
            shape1,
            pos1,
            shape2,
            pos2
        );

        let pos1 = &Vector2D::new(100.0, 100.0);
        let pos2 = &Vector2D::new(130.0, 140.0);

        assert!(
            Shape::intersects(shape1, pos1, shape2, pos2),
            "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
            shape1,
            pos1,
            shape2,
            pos2
        );

        let pos1 = &Vector2D::new(100.0, 100.0);
        let pos2 = &Vector2D::new(160.0, 100.0);

        assert!(
            !Shape::intersects(shape1, pos1, shape2, pos2),
            "Test Failed! {:?} at {:?} should not intersect {:?} at {:?}",
            shape1,
            pos1,
            shape2,
            pos2
        );
    }

    #[test]
    fn rectangle_circle_intersection_test() {
        let shape1 = &Rect {
            w: 30.0,
            h: 50.0,
            color: Color::RGB(255, 255, 255),
        };
        let shape2 = &Circle {
            r: 30.0,
            color: Color::RGB(255, 0, 0),
        };

        let pos1 = &Vector2D::new(100.0, 100.0);
        let pos2 = &Vector2D::new(140.0, 140.0);
        assert!(
            Shape::intersects(shape1, pos1, shape2, pos2),
            "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
            shape1,
            pos1,
            shape2,
            pos2
        );

        let pos1 = &Vector2D::new(50.0, 600.0);
        assert!(
            !Shape::intersects(shape1, pos1, shape2, pos2),
            "Test Failed! {:?} at {:?} should not intersect {:?} at {:?}",
            shape1,
            pos1,
            shape2,
            pos2
        );
    }
}
