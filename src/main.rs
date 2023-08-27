#![allow(dead_code, unused)]

use std::f64::consts::PI;

use argmin::core::{CostFunction, Error, Gradient};

fn main() {
    let mut system = System::new();

    let pt_a_id = system.add_point(0.0, 0.0);
    let pt_b_id = system.add_point(1.0, 1.0);
    let pt_c_id = system.add_point(1.0, 0.0);

    let spring_a_id = system.add_spring(pt_b_id, pt_c_id, 1.0);
    let spring_b_id = system.add_spring(pt_a_id, pt_c_id, 1.0);
    let spring_c_id = system.add_spring(pt_a_id, pt_b_id, 1.0);

    let spring_d_id = system.add_torsion(pt_a_id, pt_b_id, PI / 2.0);

    system.solve(100);
}

pub enum SpringKind {
    Torsion,
    Length,
}

struct Spring {
    kp: f64,   // kp is the proportional gain, the spring constant
    kd: f64,   // kd is the derivative gain, the damping constant
    rest: f64, // length is the rest length of the spring
    start_id: u64,
    end_id: u64,
    kind: SpringKind,
}

impl Spring {
    fn new(start_id: u64, end_id: u64, rest: f64, kind: SpringKind) -> Self {
        Spring {
            kp: 2.0,
            kd: 0.3,
            rest,
            start_id,
            end_id,
            kind,
        }
    }

    fn compute_forces(&self, point_a: &Point, point_b: &Point) -> Vec<f64> {
        match self.kind {
            SpringKind::Length => self.compute_length_forces(point_a, point_b),
            SpringKind::Torsion => self.compute_torsion_forces(point_a, point_b),
        }
    }

    fn compute_torsion_forces(&self, point_a: &Point, point_b: &Point) -> Vec<f64> {
        let dt = 0.01;

        let angle = (point_b.y - point_a.y).atan2(point_b.x - point_a.x);

        // println!("current angle: {}", angle);
        // println!("rest angle: {}", self.resto);
        let err = self.rest - angle;

        let point_a_stepped = point_a.step(dt);
        let point_b_stepped = point_b.step(dt);
        let angle_stepped =
            (point_b_stepped.1 - point_a_stepped.1).atan2(point_b_stepped.0 - point_a_stepped.0);
        // println!("angle_stepped: {}", angle_stepped);
        let d_angle = (angle_stepped - angle) / dt;
        // println!("d_angle: {}", d_angle);
        // let torque = self.kp * err + self.kd * d_angle;
        let torque = self.kp * err - self.kd * d_angle;

        let dx = point_b.x - point_a.x;
        let dy = point_b.y - point_a.y;
        let dist = dx.hypot(dy);
        // println!("dist: {}", dist);

        let f_mag = torque / dist;
        // println!("f_mag: {}", f_mag);

        let fx = f_mag * dy;
        let fy = -f_mag * dx;
        // println!("fx: {}", fx);
        // println!("fy: {}", fy);

        vec![fx, fy, -fx, -fy]
    }

    fn compute_length_forces(&self, point_a: &Point, point_b: &Point) -> Vec<f64> {
        let dx = point_b.x - point_a.x;
        let dy = point_b.y - point_a.y;
        let dist = (dx * dx + dy * dy).sqrt();
        let err = dist - self.rest;

        let relative_dx = point_b.dx - point_a.dx;
        let relative_dy = point_b.dy - point_a.dy;

        // project the relative velocity onto the vector between the points
        // a is the velocity
        // b is the vector between the points
        // a dot b / |b|
        let closing_velocity = (relative_dx * dx + relative_dy * dy) / dist;

        let f = self.kp * err + self.kd * closing_velocity;
        let fx = f * dx / dist;
        let fy = f * dy / dist;

        // [fx_a, fy_a, fx_b, fy_b]
        vec![fx, fy, -fx, -fy]
    }
}

struct Point {
    x: f64,
    y: f64,
    m: f64,
    dx: f64,
    dy: f64,
    fx: f64,
    fy: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point {
            x,
            y,
            m: 1.0,
            dx: 0.0,
            dy: 0.0,
            fx: 0.0,
            fy: 0.0,
        }
    }

    fn reset_forces(&mut self) {
        self.fx = 0.0;
        self.fy = 0.0;
    }

    fn step(&self, dt: f64) -> (f64, f64) {
        (self.x + self.dx * dt, self.y + self.dy * dt)
    }
}

struct System {
    points: Vec<Point>,
    springs: Vec<Spring>,
}

impl System {
    fn new() -> Self {
        System {
            points: Vec::new(),
            springs: Vec::new(),
        }
    }

    fn add_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.points.len();
        self.points.push(Point::new(x, y));
        id as u64
    }

    fn add_spring(&mut self, start_id: u64, end_id: u64, length: f64) -> u64 {
        let id = self.springs.len();
        let s = Spring::new(start_id, end_id, length, SpringKind::Length);
        self.springs.push(s);
        id as u64
    }

    fn add_torsion(&mut self, start_id: u64, end_id: u64, angle: f64) -> u64 {
        let id = self.springs.len();
        self.springs
            .push(Spring::new(start_id, end_id, angle, SpringKind::Torsion));
        id as u64
    }

    fn solve(&mut self, steps: u64) {
        self.print_state();
        for _ in 0..steps {
            self.step();
            self.print_state();
        }
    }

    fn step(&mut self) {
        let dt = 0.04;
        for point in self.points.iter_mut() {
            point.reset_forces();
        }
        for spring in self.springs.iter() {
            let point_a = &self.points[spring.start_id as usize];
            let point_b = &self.points[spring.end_id as usize];
            let forces = spring.compute_forces(point_a, point_b);

            self.points[spring.start_id as usize].fx += forces[0];
            self.points[spring.start_id as usize].fy += forces[1];
            self.points[spring.end_id as usize].fx += forces[2];
            self.points[spring.end_id as usize].fy += forces[3];
        }
        for point in self.points.iter_mut() {
            let ax = point.fx / point.m;
            let ay = point.fy / point.m;
            point.dx += ax;
            point.dy += ay;
            point.x += 0.5 * ax * dt * dt + point.dx * dt;
            point.y += 0.5 * ay * dt * dt + point.dy * dt;
        }
    }

    fn print_state(&self) {
        let mut data = vec![];
        for point in self.points.iter() {
            data.push(point.x);
            data.push(point.y);
            data.push(point.dx);
            data.push(point.dy);
            data.push(point.fx);
            data.push(point.fy);
        }
        let mut strings = data.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        println!("{}", strings.join(","));
    }
}

fn main_one_var() {
    let mut x = 0.0;
    let z = 1.0;

    let k = 100.0; // proportional
    let k2 = -12.0; // derivative
    let dt = 0.04;
    let m = 1.0;
    let mut v = 0.0;
    // println!("{:?}", x);
    for i in 0..100 {
        let err = z - x;
        let f = k * err + k2 * v;
        let a = f / m;

        let dv = a * dt;
        v += dv;

        let dx = 0.5 * a * dt * dt + v * dt;
        x += dx;
        println!("{:?}", err);
    }
}
