use std::f64::consts::PI;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Rect { w: f64, h: f64 },
    Circle { r: f64 },
    Polygon { points: Vec<Vector2D<f64>> },
}

pub struct CollisionData {
    pub collision_point: Vector2D<f64>,
    pub normal_vector: Vector2D<f64>,
    pub depth: f64,
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { r, .. } => PI * r.powi(2),
            Shape::Rect { w, h, .. } => w * h,
            Shape::Polygon { .. } => todo!(),
        }
    }

    pub fn display(
        &self,
        canvas: &Canvas<Window>,
        pos: &Vector2D<f64>,
        _rot: f64,
        color: &Color,
    ) -> Result<(), String> {
        match self {
            Shape::Rect { w, h } => canvas.filled_polygon(
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
            ),
            Shape::Circle { r } => {
                canvas.filled_circle(pos.x as i16, pos.y as i16, *r as i16, *color)
            }
            Shape::Polygon { points } => {
                let (vx, vy): (Vec<_>, Vec<_>) = points
                    .iter()
                    .map(|vector| ((vector.x + pos.x) as i16, (vector.y + pos.y) as i16))
                    .unzip();
                canvas.filled_polygon(&vx, &vy, *color)
            }
        }
    }

    pub fn get_aabb(&self) -> Shape {
        match self {
            Shape::Rect { w, h } => Shape::Rect { w: *w, h: *h },
            Shape::Circle { r } => Shape::Rect {
                w: 2.0 * r,
                h: 2.0 * r,
            },
            Shape::Polygon { .. } => todo!(),
        }
    }

    pub fn point_inside(&self, offset: &Vector2D<f64>, point: &Vector2D<f64>) -> bool {
        match self {
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
            Shape::Polygon { .. } => todo!(),
        }
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
            (Shape::Polygon { .. }, _) => todo!(),
            (shape1, shape2) => Shape::intersects(shape2, pos2, shape1, pos1),
        }
    }

    // undefined behaviour for
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
                        depth: upper_corner_y - lower_corner_y,
                    }
                } else {
                    CollisionData {
                        collision_point,
                        normal_vector: Vector2D::new(if pos1.x < pos2.x { 1.0 } else { -1.0 }, 0.0),
                        depth: upper_corner_x - lower_corner_x,
                    }
                }
            }
            (Shape::Rect { w, h, .. }, Shape::Circle { r, .. }) => {
                //FIXME This entire function is some major whack
                let close_x = pos2.x.max(pos1.x - w / 2.0).min(pos1.x + w / 2.0);
                let close_y = pos2.y.max(pos1.y - h / 2.0).min(pos1.y + h / 2.0);
                let close = &Vector2D::new(close_x, close_y);

                if close == pos2 {
                    //Circle inside rectangle
                    let diff = pos2 - pos1;
                    let x_side_dist = (diff.x.abs() - w / 2.0).abs();
                    let y_side_dist = (diff.y.abs() - h / 2.0).abs();

                    if x_side_dist < y_side_dist {
                        //X direction
                        let edge_point = Vector2D::new(pos1.x + diff.x.signum() * w / 2.0, pos2.y);
                        let innermost_point = pos2 + &Vector2D::new(diff.x.signum() * -r, 0.0);
                        CollisionData {
                            collision_point: (edge_point + innermost_point) / 2.0,
                            depth: (innermost_point - edge_point).length(),
                            normal_vector: Vector2D::new(diff.x.signum(), 0.0),
                        }
                    } else {
                        //Y direction
                        let edge_point = Vector2D::new(pos2.x, pos1.y + diff.y.signum() * h / 2.0);
                        let innermost_point = pos2 + &Vector2D::new(0.0, diff.y.signum() * -r);
                        CollisionData {
                            collision_point: (edge_point + innermost_point) / 2.0,
                            depth: (innermost_point - edge_point).length(),
                            normal_vector: Vector2D::new(0.0, diff.y.signum()),
                        }
                    }
                } else {
                    // Circle outside of rectangle
                    let depth = r - (pos2 - close).length();
                    let normal_vector = (pos2 - close).normalise();
                    let innermost_point = pos2 - &(normal_vector * *r);
                    CollisionData {
                        collision_point: (close + &innermost_point) / 2.0,
                        normal_vector,
                        depth,
                    }
                }
            }
            (Shape::Circle { r: r1, .. }, Shape::Circle { r: r2, .. }) => {
                let diff = pos2 - pos1;
                let overlap = (r1 + r2) - diff.length();
                let norm = diff.normalise();
                CollisionData {
                    collision_point: pos1 + &(norm * (overlap / 2.0 + r1)),
                    normal_vector: norm,
                    depth: overlap,
                }
            }
            (Shape::Polygon { .. }, _) => todo!(),
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
    use vector2d::Vector2D;

    use super::Shape::{self, Circle, Rect};
    #[test]
    fn test_rectangle_intersection() {
        let shape1 = &Rect { w: 20.0, h: 30.0 };
        let shape2 = &Rect { w: 50.0, h: 10.0 };
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
    fn test_circle_intersection() {
        let shape1 = &Circle { r: 30.0 };
        let shape2 = &Circle { r: 20.0 };

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
    fn test_rectangle_circle_intersection() {
        let shape1 = &Rect { w: 30.0, h: 50.0 };
        let shape2 = &Circle { r: 30.0 };

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

    #[test]
    fn test_rectangle_rectangle_collision_data() {
        let shape1 = &Rect { w: 50.0, h: 40.0 };
        let shape2 = &Rect { w: 40.0, h: 60.0 };
        let pos1 = &Vector2D::new(100.0, 100.0);
        let pos2 = &Vector2D::new(100.0, 140.0);

        let collision_data = Shape::collision_data(shape1, pos1, shape2, pos2);
        let collision_data = collision_data;
        assert_eq!(collision_data.collision_point, Vector2D::new(100.0, 115.0));
        assert_eq!(collision_data.depth, 10.0);
        assert_eq!(collision_data.normal_vector, Vector2D::new(0.0, 1.0));
    }

    #[test]
    fn test_circle_circle_collision_data() {
        let shape1 = &Circle { r: 40.0 };
        let shape2 = &Circle { r: 40.0 };

        let pos1 = &Vector2D::new(0.0, 0.0);
        let pos2 = &Vector2D::new(30.0, 40.0);

        let collision_data = Shape::collision_data(shape1, pos1, shape2, pos2);
        let collision_data = collision_data;
        assert_eq!(
            collision_data.collision_point,
            Vector2D::new(30.0 + 3.0, 40.0 + 4.0)
        );
        assert_eq!(collision_data.depth, 30.0);
        assert_eq!(
            collision_data.normal_vector,
            Vector2D::new(3.0 / 5.0, 4.0 / 5.0)
        );
    }

    fn test_collision_intersection_data(
        shape1: &Shape,
        pos1: &Vector2D<f64>,
        shape2: &Shape,
        pos2: &Vector2D<f64>,
        collision_point: &Vector2D<f64>,
        normal_vector: &Vector2D<f64>,
        depth: f64,
    ) {
        let collision_data = Shape::collision_data(shape1, pos1, shape2, pos2);
        let collision_data = collision_data;

        assert_eq!(collision_data.collision_point, *collision_point);
        assert_eq!(collision_data.normal_vector, *normal_vector);
        assert_eq!(collision_data.depth, depth);
    }

    #[test]
    fn test_rectangle_circle_collision_data_vertical_outside() {
        test_collision_intersection_data(
            &Rect { w: 200.0, h: 50.0 },
            &Vector2D::new(0.0, 0.0),
            &Circle { r: 20.0 },
            &Vector2D::new(0.0, 40.0),
            &Vector2D::new(0.0, 22.5),
            &Vector2D::new(0.0, 1.0),
            5.0,
        );
    }

    #[test]
    fn test_rectangle_circle_collision_data_vertical_inside() {
        test_collision_intersection_data(
            &Rect { w: 200.0, h: 50.0 },
            &Vector2D::new(0.0, 0.0),
            &Circle { r: 20.0 },
            &Vector2D::new(0.0, 20.0),
            &Vector2D::new(0.0, 12.5),
            &Vector2D::new(0.0, 1.0),
            25.0,
        );
    }

    #[test]
    fn test_rectangle_circle_collision_data_horizontal_outside() {
        test_collision_intersection_data(
            &Rect { w: 50.0, h: 200.0 },
            &Vector2D::new(0.0, 0.0),
            &Circle { r: 20.0 },
            &Vector2D::new(40.0, 0.0),
            &Vector2D::new(22.5, 0.0),
            &Vector2D::new(1.0, 0.0),
            5.0,
        );
    }

    #[test]
    fn test_rectangle_circle_collision_data_horizontal_inside() {
        test_collision_intersection_data(
            &Rect { w: 50.0, h: 200.0 },
            &Vector2D::new(0.0, 0.0),
            &Circle { r: 20.0 },
            &Vector2D::new(20.0, 0.0),
            &Vector2D::new(12.5, 0.0),
            &Vector2D::new(1.0, 0.0),
            25.0,
        );
    }
}
