#![allow(dead_code, unused)]

fn main() {
    println!("Hello, world!");

    let mut system = System::new();

    let id0 = system.add_datum_point(DatumPoint::new(0.0, 0.0));
    let id1 = system.add_datum_point(DatumPoint::new(1.0, 1.0));

    system.add_point_point_distance_constraint(id0, id1, 1.0);

    println!("{:?}", id0);
    println!("{:?}", id1);
}

#[derive(Debug)]
struct DatumPoint {
    x: f64, // distance is always meters so as to always be the same order of magnitude as radians
    y: f64,
}

impl DatumPoint {
    fn new(x: f64, y: f64) -> DatumPoint {
        DatumPoint { x, y }
    }
}

struct DatumLine {
    theta: f64,  // radians: constrain to [0, pi)
    length: f64, // meters
}

struct LineSegment {
    start: DatumPoint,
    end: DatumPoint,
}

struct Circle {
    center: DatumPoint,
    radius: f64,
}

struct Arc {
    start: DatumPoint,
    end: DatumPoint,
    center: DatumPoint,
}

trait Entity {
    fn parameters(&self) -> Vec<f64>;
}

impl Entity for DatumPoint {
    fn parameters(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }
}

trait Constraint {
    fn cost(&self) -> f64;
    fn gradient(&self) -> Vec<f64>;
}

#[derive(Debug)]
struct PointPointDistanceConstraint {
    start: DatumPoint,
    end: DatumPoint,
    distance: f64,
}

impl Constraint for PointPointDistanceConstraint {
    fn cost(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let d = (dx * dx + dy * dy).sqrt();
        (d - self.distance).abs()
    }

    fn gradient(&self) -> Vec<f64> {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let d = (dx * dx + dy * dy).sqrt();
        let ddx = dx / d;
        let ddy = dy / d;
        vec![ddx, ddy, -ddx, -ddy]
    }
}

struct System {
    entities: Vec<Box<dyn Entity>>,
    constraints: Vec<Box<dyn Constraint>>,
}

impl System {
    fn new() -> System {
        System {
            entities: Vec::new(),
            constraints: Vec::new(),
        }
    }

    fn add_datum_point(&mut self, point: DatumPoint) -> usize {
        self.entities.push(Box::new(point));
        let id = self.entities.len() - 1;
        id
    }

    fn add_point_point_distance_constraint(&mut self, id0: usize, id1: usize, distance: f64) {}
}
