use std::f64::consts::PI;
use std::f64::INFINITY;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
enum ShapeInner {
    Circle { r: f64 },
    Polygon { points: Vec<Vector2D<f64>> },
}
// TODO move shape collision data here

#[derive(Debug, PartialEq)]
pub struct Shape(ShapeInner);

impl Shape {
    pub fn new_poly(ps: Vec<Vector2D<f64>>) -> Self {
        for i in 0..ps.len() {
            let p1 = ps[i];
            let p2 = ps[(i + 1) % ps.len()];
            let p3 = ps[(i + 2) % ps.len()];
            let angle = ((p3 - p2).angle() - (p2 - p1).angle()).rem_euclid(2.0 * PI);

            assert!(
                angle > 0.0 && angle < PI,
                "POLYGON NOT CLOCKWISE: Angle between points needs to be 0 < x < pi but was {:.2}",
                angle
            );
        }
        Shape(ShapeInner::Polygon { points: ps })
    }

    pub fn new_rect(w: f64, h: f64) -> Self {
        let points = vec![
            Vector2D::new(-w / 2.0, -h / 2.0),
            Vector2D::new(w / 2.0, -h / 2.0),
            Vector2D::new(w / 2.0, h / 2.0),
            Vector2D::new(-w / 2.0, h / 2.0),
        ];
        Shape(ShapeInner::Polygon { points })
    }

    pub fn new_circle(r: f64) -> Self {
        Shape(ShapeInner::Circle { r })
    }
}

pub struct AABB {
    min: Vector2D<f64>,
    max: Vector2D<f64>,
}
impl AABB {
    pub fn intersects(box1: &AABB, box2: &AABB) -> bool {
        box1.max.x >= box2.min.x
            && box1.max.y >= box2.min.y
            && box2.max.x >= box1.min.x
            && box2.max.y >= box1.min.x
    }
}

pub struct CollisionData {
    pub collision_point: Vector2D<f64>,
    pub normal_vector: Vector2D<f64>,
    pub depth: f64,
}
impl CollisionData {
    pub fn display(&self, canvas: &Canvas<Window>) {
        let p1 = self.collision_point - self.normal_vector * self.depth / 2.0;
        let p2 = self.collision_point + self.normal_vector * self.depth / 2.0;
        canvas
            .line(
                p1.x as i16,
                p1.y as i16,
                p2.x as i16,
                p2.y as i16,
                Color::WHITE,
            )
            .unwrap();
        canvas
            .circle(
                self.collision_point.x as i16,
                self.collision_point.y as i16,
                5,
                Color::WHITE,
            )
            .unwrap();
    }
}

impl Shape {
    pub fn area(&self) -> f64 {
        match &self.0 {
            ShapeInner::Circle { r, .. } => PI * r.powi(2),
            ShapeInner::Polygon { points } => {
                //TODO: TEST THIS!
                fn triangle_area(points: &[Vector2D<f64>; 3]) -> f64{
                    let e1 = points[0] - points[1];
                    let height_normal = e1.normal().normalise();
                    let e2 = points[2] - points[1];

                    (Vector2D::dot(height_normal, e2) * e1.length()).abs()
                }
                let mut area = 0.0;
                let p1 = points[0];
                for i in 0..points.len()-2{
                    let p2 = points[i+1];
                    let p3 = points[i+2];
                    area += triangle_area(&[p1, p2, p3]);
                }
                area
            },
        }
    }

    pub fn display(
        &self,
        canvas: &Canvas<Window>,
        pos: &Vector2D<f64>,
        _rot: f64,
        color: &Color,
    ) -> Result<(), String> {
        match &self.0 {
            ShapeInner::Circle { r } => {
                canvas.filled_circle(pos.x as i16, pos.y as i16, *r as i16, *color)
            }
            ShapeInner::Polygon { points } => {
                let (vx, vy): (Vec<_>, Vec<_>) = points
                    .iter()
                    .map(|vector| ((vector.x + pos.x) as i16, (vector.y + pos.y) as i16))
                    .unzip();
                canvas.filled_polygon(&vx, &vy, *color)
            }
        }
    }

    pub fn get_aabb(&self, pos: Vector2D<f64>, rot: f64) -> AABB {
        match &self.0 {
            ShapeInner::Circle { r } => AABB {
                min: pos - Vector2D::new(*r, *r),
                max: pos + Vector2D::new(*r, *r),
            },
            ShapeInner::Polygon { points } => {
                let ps = &points.iter().map(|p| p + &pos).collect::<Vec<_>>();
                let mut min = Vector2D::new(INFINITY, INFINITY);
                let mut max = Vector2D::new(-INFINITY, -INFINITY);
                for p in ps {
                    min.x = min.x.min(p.x);
                    min.y = min.y.min(p.y);
                    max.x = max.x.max(p.x);
                    max.y = max.y.max(p.y);
                }
                AABB { min, max }
            }
        }
    }

    pub fn point_inside(&self, pos: &Vector2D<f64>, point: &Vector2D<f64>) -> bool {
        match &self.0 {
            ShapeInner::Circle { r } => {
                let dist = (pos - point).length_squared();
                dist < r.powi(2)
            }
            ShapeInner::Polygon { points } => {
                //This is mad
                let ps = &points.iter().map(|p| p + pos).collect::<Vec<_>>();
                for i in 0..ps.len() {
                    let p1 = ps[i];
                    let p2 = ps[(i + 1) % ps.len()];
                    let angle = ((p2 - p1).angle() - (p1 - *point).angle()).rem_euclid(2.0 * PI);

                    if !(0.0..=PI).contains(&angle) {
                        return false;
                    }
                }
                true
            },
        }
    }

    // TODO delete
    pub fn intersects(
        shape1: &Shape,
        pos1: &Vector2D<f64>,
        shape2: &Shape,
        pos2: &Vector2D<f64>,
    ) -> bool {
        match (shape1, shape2) {
            (Shape(ShapeInner::Circle { r: r1, .. }), Shape(ShapeInner::Circle { r: r2, .. })) => {
                let dist = (pos1 - pos2).length_squared();
                dist <= (r1 + r2).powi(2)
            }
            (
                Shape(ShapeInner::Polygon { points: points1 }),
                Shape(ShapeInner::Polygon { points: points2 }),
            ) => {
                let ps1 = &points1.iter().map(|p| p + pos1).collect::<Vec<_>>();
                let ps2 = &points2.iter().map(|p| p + pos2).collect::<Vec<_>>();
                fn sat(ps1: &Vec<Vector2D<f64>>, ps2: &Vec<Vector2D<f64>>) -> bool {
                    for i in 0..ps1.len() {
                        let p1 = ps1[i];
                        let p2 = ps1[(i + 1) % ps1.len()];
                        let n = Vector2D::new(p1.y - p2.y, p2.x - p1.x);

                        let mut min_dist = INFINITY;
                        for &q in ps2 {
                            let dist = Vector2D::dot(n, p1 - q);
                            min_dist = min_dist.min(dist);
                        }

                        if min_dist > 0.0 {
                            return false;
                        }
                    }
                    true
                }
                sat(ps1, ps2) && sat(ps2, ps1)
            }
            (Shape(ShapeInner::Polygon { points }), Shape(ShapeInner::Circle { r })) => {
                let ps = &points.iter().map(|p| p + pos1).collect::<Vec<_>>();

                for i in 0..ps.len() {
                    let p1 = ps[i];
                    let p2 = ps[(i + 1) % ps.len()];
                    let n = Vector2D::new(p1.y - p2.y, p2.x - p1.x).normalise();

                    let dist = Vector2D::dot(n, p1 - *pos2);

                    if dist > *r {
                        return false;
                    }
                }

                for p in ps {
                    let n = (p - pos2).normalise();

                    let mut min_dist = INFINITY;
                    for &q in ps {
                        let dist = Vector2D::dot(n, q - *pos2);
                        min_dist = min_dist.min(dist);
                    }

                    if min_dist > *r {
                        return false
                    }
                }

                true
            }
            (shape1, shape2) => Shape::intersects(shape2, pos2, shape1, pos1),
        }
    }

    // TODO replace with an option
    pub fn collision_data(
        shape1: &Shape,
        pos1: &Vector2D<f64>,
        shape2: &Shape,
        pos2: &Vector2D<f64>,
    ) -> CollisionData {
        match (shape1, shape2) {
            (Shape(ShapeInner::Circle { r: r1 }), Shape(ShapeInner::Circle { r: r2 })) => {
                let diff = pos2 - pos1;
                let overlap = (r1 + r2) - diff.length();
                let norm = diff.normalise();
                CollisionData {
                    collision_point: pos1 + &(norm * (overlap / 2.0 + r1)),
                    normal_vector: norm,
                    depth: overlap,
                }
            }

            (
                Shape(ShapeInner::Polygon { points }),
                Shape(ShapeInner::Circle { r }),
            ) => {
                let ps = &points.iter().map(|p| p + pos1).collect::<Vec<_>>();

                let mut collision_point = Vector2D::new(0.0, 0.0);
                let mut normal_vector = Vector2D::new(0.0, 0.0);
                let mut depth = INFINITY;

                for i in 0..ps.len() {
                    let n = (ps[i] - ps[(i + 1) % ps.len()]).normal().normalise();

                    let deepest_point = pos2 - &(n* *r);
                    let d = -Vector2D::dot(n, deepest_point - ps[i]);
                    if d <= depth {
                        collision_point = deepest_point + n*d / 2.0;
                        depth = d;
                        normal_vector = n;
                    }
                }

                for i in 0..ps.len() {
                    let p0 = ps[i];
                    let p1 = ps[(i+1)%ps.len()];
                    let p2 = ps[(i+2)%ps.len()];

                    let s0 = p1 - p0;
                    let s1 = p2 - p1;

                    let l0 = s0.length();
                    let l1 = s1.length();

                    let dot0 = Vector2D::dot(s0, pos2 - &p0)/l0.powi(2);
                    let dot1 = Vector2D::dot(s1, pos2 - &p1)/l1.powi(2);

                    if dot0 < 1.0 || 0.0 < dot1 {
                        continue;
                    }

                    let d = *r - (pos2 - &p1).length();
                    let n = -(pos2 - &p1).normalise();
                    if 0.0 < d && d <= depth {
                        collision_point = p1 + n * d / 2.0;
                        depth = d;
                        normal_vector = -n;
                    }
                }

                CollisionData {
                    collision_point,
                    normal_vector,
                    depth,
                }
            }

            (
                Shape(ShapeInner::Polygon { points: points1 }),
                Shape(ShapeInner::Polygon { points: points2 }),
            ) => {
                let ps1 = &points1.iter().map(|p| p + pos1).collect::<Vec<_>>();
                let ps2 = &points2.iter().map(|p| p + pos2).collect::<Vec<_>>();

                let mut collision_point = Vector2D::new(0.0, 0.0);
                let mut normal_vector = Vector2D::new(0.0, 0.0);
                let mut depth = INFINITY;

                for i in 0..ps1.len() {
                    let n = (ps1[i] - ps1[(i + 1) % ps1.len()]).normal().normalise();
                    let mut deepest_j = 0;
                    for j in 1..ps2.len() {
                        if Vector2D::dot(n, ps2[deepest_j]) > Vector2D::dot(n, ps2[j]) {
                            deepest_j = j;
                        }
                    }

                    let d = -Vector2D::dot(n, ps2[deepest_j] - ps1[i]);
                    if d <= depth {
                        collision_point = ps2[deepest_j] + n * d / 2.0;
                        depth = d;
                        normal_vector = n;
                    }
                }
                for i in 0..ps2.len() {
                    let n = (ps2[i] - ps2[(i + 1) % ps2.len()]).normal().normalise();
                    let mut deepest_j = 0;
                    for j in 1..ps1.len() {
                        if Vector2D::dot(n, ps1[deepest_j]) > Vector2D::dot(n, ps1[j]) {
                            deepest_j = j;
                        }
                    }

                    let d = -Vector2D::dot(n, ps1[deepest_j] - ps2[i]);
                    if d <= depth {
                        collision_point = ps1[deepest_j] + n * d / 2.0;
                        depth = d;
                        normal_vector = -n;
                    }
                }
                CollisionData {
                    collision_point,
                    normal_vector,
                    depth,
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


//TODO: Rewrite tests
// #[cfg(test)]
// mod tests {
//     use vector2d::Vector2D;

//     use super::Shape::{self, Circle, Rect};
//     #[test]
//     fn test_rectangle_intersection() {
//         let shape1 = &Rect { w: 20.0, h: 30.0 };
//         let shape2 = &Rect { w: 50.0, h: 10.0 };
//         let pos1 = &Vector2D::new(100.0, 100.0);
//         let pos2 = &Vector2D::new(130.0, 110.0);

//         assert!(
//             Shape::intersects(shape1, pos1, shape2, pos2),
//             "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
//             shape1,
//             pos1,
//             shape2,
//             pos2
//         );

//         let pos1 = &Vector2D::new(100.0, 100.0);
//         let pos2 = &Vector2D::new(135.0, 115.0);

//         assert!(
//             Shape::intersects(shape1, pos1, shape2, pos2),
//             "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
//             shape1,
//             pos1,
//             shape2,
//             pos2
//         );

//         let pos1 = &Vector2D::new(100.0, 100.0);
//         let pos2 = &Vector2D::new(150.0, 130.0);

//         assert!(
//             !Shape::intersects(shape1, pos1, shape2, pos2),
//             "Test Failed! {:?} at {:?} should not intersect {:?} at {:?}",
//             shape1,
//             pos1,
//             shape2,
//             pos2
//         );
//     }

//     #[test]
//     fn test_circle_intersection() {
//         let shape1 = &Circle { r: 30.0 };
//         let shape2 = &Circle { r: 20.0 };

//         let pos1 = &Vector2D::new(100.0, 100.0);
//         let pos2 = &Vector2D::new(130.0, 100.0);

//         assert!(
//             Shape::intersects(shape1, pos1, shape2, pos2),
//             "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
//             shape1,
//             pos1,
//             shape2,
//             pos2
//         );

//         let pos1 = &Vector2D::new(100.0, 100.0);
//         let pos2 = &Vector2D::new(130.0, 140.0);

//         assert!(
//             Shape::intersects(shape1, pos1, shape2, pos2),
//             "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
//             shape1,
//             pos1,
//             shape2,
//             pos2
//         );

//         let pos1 = &Vector2D::new(100.0, 100.0);
//         let pos2 = &Vector2D::new(160.0, 100.0);

//         assert!(
//             !Shape::intersects(shape1, pos1, shape2, pos2),
//             "Test Failed! {:?} at {:?} should not intersect {:?} at {:?}",
//             shape1,
//             pos1,
//             shape2,
//             pos2
//         );
//     }

//     #[test]
//     fn test_rectangle_circle_intersection() {
//         let shape1 = &Rect { w: 30.0, h: 50.0 };
//         let shape2 = &Circle { r: 30.0 };

//         let pos1 = &Vector2D::new(100.0, 100.0);
//         let pos2 = &Vector2D::new(140.0, 140.0);
//         assert!(
//             Shape::intersects(shape1, pos1, shape2, pos2),
//             "Test Failed! {:?} at {:?} should intersect {:?} at {:?}",
//             shape1,
//             pos1,
//             shape2,
//             pos2
//         );

//         let pos1 = &Vector2D::new(50.0, 600.0);
//         assert!(
//             !Shape::intersects(shape1, pos1, shape2, pos2),
//             "Test Failed! {:?} at {:?} should not intersect {:?} at {:?}",
//             shape1,
//             pos1,
//             shape2,
//             pos2
//         );
//     }

//     #[test]
//     fn test_rectangle_rectangle_collision_data() {
//         let shape1 = &Rect { w: 50.0, h: 40.0 };
//         let shape2 = &Rect { w: 40.0, h: 60.0 };
//         let pos1 = &Vector2D::new(100.0, 100.0);
//         let pos2 = &Vector2D::new(100.0, 140.0);

//         let collision_data = Shape::collision_data(shape1, pos1, shape2, pos2);
//         let collision_data = collision_data;
//         assert_eq!(collision_data.collision_point, Vector2D::new(100.0, 115.0));
//         assert_eq!(collision_data.depth, 10.0);
//         assert_eq!(collision_data.normal_vector, Vector2D::new(0.0, 1.0));
//     }

//     #[test]
//     fn test_circle_circle_collision_data() {
//         let shape1 = &Circle { r: 40.0 };
//         let shape2 = &Circle { r: 40.0 };

//         let pos1 = &Vector2D::new(0.0, 0.0);
//         let pos2 = &Vector2D::new(30.0, 40.0);

//         let collision_data = Shape::collision_data(shape1, pos1, shape2, pos2);
//         let collision_data = collision_data;
//         assert_eq!(
//             collision_data.collision_point,
//             Vector2D::new(30.0 + 3.0, 40.0 + 4.0)
//         );
//         assert_eq!(collision_data.depth, 30.0);
//         assert_eq!(
//             collision_data.normal_vector,
//             Vector2D::new(3.0 / 5.0, 4.0 / 5.0)
//         );
//     }

//     fn test_collision_intersection_data(
//         shape1: &Shape,
//         pos1: &Vector2D<f64>,
//         shape2: &Shape,
//         pos2: &Vector2D<f64>,
//         collision_point: &Vector2D<f64>,
//         normal_vector: &Vector2D<f64>,
//         depth: f64,
//     ) {
//         let collision_data = Shape::collision_data(shape1, pos1, shape2, pos2);
//         let collision_data = collision_data;

//         assert_eq!(collision_data.collision_point, *collision_point);
//         assert_eq!(collision_data.normal_vector, *normal_vector);
//         assert_eq!(collision_data.depth, depth);
//     }

//     #[test]
//     fn test_rectangle_circle_collision_data_vertical_outside() {
//         test_collision_intersection_data(
//             &Rect { w: 200.0, h: 50.0 },
//             &Vector2D::new(0.0, 0.0),
//             &Circle { r: 20.0 },
//             &Vector2D::new(0.0, 40.0),
//             &Vector2D::new(0.0, 22.5),
//             &Vector2D::new(0.0, 1.0),
//             5.0,
//         );
//     }

//     #[test]
//     fn test_rectangle_circle_collision_data_vertical_inside() {
//         test_collision_intersection_data(
//             &Rect { w: 200.0, h: 50.0 },
//             &Vector2D::new(0.0, 0.0),
//             &Circle { r: 20.0 },
//             &Vector2D::new(0.0, 20.0),
//             &Vector2D::new(0.0, 12.5),
//             &Vector2D::new(0.0, 1.0),
//             25.0,
//         );
//     }

//     #[test]
//     fn test_rectangle_circle_collision_data_horizontal_outside() {
//         test_collision_intersection_data(
//             &Rect { w: 50.0, h: 200.0 },
//             &Vector2D::new(0.0, 0.0),
//             &Circle { r: 20.0 },
//             &Vector2D::new(40.0, 0.0),
//             &Vector2D::new(22.5, 0.0),
//             &Vector2D::new(1.0, 0.0),
//             5.0,
//         );
//     }

//     #[test]
//     fn test_rectangle_circle_collision_data_horizontal_inside() {
//         test_collision_intersection_data(
//             &Rect { w: 50.0, h: 200.0 },
//             &Vector2D::new(0.0, 0.0),
//             &Circle { r: 20.0 },
//             &Vector2D::new(20.0, 0.0),
//             &Vector2D::new(12.5, 0.0),
//             &Vector2D::new(1.0, 0.0),
//             25.0,
//         );
//     }
// }
