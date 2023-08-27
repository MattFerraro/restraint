#![allow(dead_code, unused)]

use argmin::core::{CostFunction, Error, Gradient};

fn main() {
    println!("Hello, world!");

    // Start with the simplest possible system: two points, one constraint
    let mut system = System::new();
    let id0 = system.add_point(0.0, 0.0);
    let id1 = system.add_point(1.0, 1.0);
    system.add_point_point_distance_constraint(id0, id1, 1.0);

    println!("Simplest possible:");
    println!("{:?}", system.error());
    println!("{:?}", system.gradient());

    // Expand it to be a triangle
    let id2 = system.add_point(1.0, 0.0);
    system.add_point_point_distance_constraint(id0, id2, 1.0);
    system.add_point_point_distance_constraint(id1, id2, 1.0);
    println!("Triangle:");
    println!("{:?}", system.error());
    println!("{:?}", system.gradient());
}

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct DatumLine {
    theta: f64,
    length: f64,
}

#[derive(Debug)]
struct Entity {
    id: u64,
    kind: EntityKind,
    points: Vec<Point>,
    parameters: Vec<f64>,
    lines: Vec<DatumLine>,
}

#[derive(Debug)]
enum EntityKind {
    Point,
    Line,
    Circle,
    Arc,
}

// #[derive(Debug)]
// enum Constraint {
//     PointPointDistance {
//         start_id: u64,
//         end_id: u64,
//         distance: f64,
//     },
// }

trait Constraint {
    fn error(&self, system: &System) -> f64;

    // the u64 is the id of the entity, the f64s are the partial derivatives
    // of the error with respect to the parameters of the entity
    fn gradient(&self, system: &System) -> Vec<(u64, f64, f64)>;
}

#[derive(Debug)]
struct PointPointDistance {
    start_id: u64,
    end_id: u64,
    distance: f64,
}

impl Constraint for PointPointDistance {
    fn error(&self, system: &System) -> f64 {
        let start: Point = system.entities[self.start_id as usize].points[0].clone();
        let end: Point = system.entities[self.end_id as usize].points[0].clone();

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let d = (dx * dx + dy * dy).sqrt();
        (d - self.distance).abs() // .abs()? maybe should square?
    }

    fn gradient(&self, system: &System) -> Vec<(u64, f64, f64)> {
        let start: Point = system.entities[self.start_id as usize].points[0].clone();
        let end: Point = system.entities[self.end_id as usize].points[0].clone();

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let d = (dx * dx + dy * dy).sqrt();
        let ddx = dx / d;
        let ddy = dy / d;
        vec![(self.start_id, ddx, ddy), (self.end_id, -ddx, -ddy)]
    }
}

struct System {
    entities: Vec<Entity>,
    constraints: Vec<Box<dyn Constraint>>,
}

impl System {
    fn new() -> System {
        System {
            entities: Vec::new(),
            constraints: Vec::new(),
        }
    }

    fn add_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.entities.len() as u64;
        let entity = Entity {
            points: vec![Point { x, y }],
            parameters: Vec::new(),
            lines: Vec::new(),
            id,
            kind: EntityKind::Point,
        };
        self.entities.push(entity);
        id
    }

    fn add_point_point_distance_constraint(&mut self, id0: u64, id1: u64, distance: f64) {
        let constraint = PointPointDistance {
            start_id: id0,
            end_id: id1,
            distance,
        };
        self.constraints.push(Box::new(constraint));
    }

    fn error(&self) -> f64 {
        let mut error = 0.0;
        for constraint in &self.constraints {
            let constraint_error = constraint.error(&self);
            error += constraint_error;
            // println!("c error: {:?}", constraint.error(&self));
        }
        error
    }

    fn gradient(&self) -> Vec<f64> {
        // this is the final, actual gradient. It is as long as the number of parameters in the system.
        let mut gradient: Vec<f64> = Vec::new();
        let mut all_gradients = &self
            .constraints
            .iter()
            .map(|c| c.gradient(&self))
            .flatten()
            .collect::<Vec<_>>();

        println!("all_gradients: {:?}", all_gradients);
        for (id, dx, dy) in all_gradients.iter() {
            println!("id: {:?}, dx: {:?}, dy: {:?}", id, dx, dy);
        }
        gradient
    }
}

impl CostFunction for System {
    type Param = System;
    type Output = f64;

    /// Apply the cost function to a parameter `p`
    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        let cost = self.error();
        Ok(cost)
    }
}

impl Gradient for System {
    type Param = System;
    type Gradient = Vec<f64>;

    /// Compute the gradient at parameter `p`.
    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient, Error> {
        // Compute gradient of 2D Rosenbrock function
        Ok(self.gradient())
    }
}
