//arcs, lines, and points

use std::ops::{
    Mul,
    Add,
    Sub
};
use std::f64;

#[derive(Copy,Clone,Debug)]
pub struct Point
{
    pub x:f64,
    pub y:f64
}

impl Point
{
    fn dist(self, other: Point) -> f64
    {
        let delta = self - other;
        f64:: sqrt(delta*delta)
    }
}

impl Mul for Point
{
    type Output = f64;
    fn mul(self, rhs: Point) -> f64
    {
        self.x*rhs.x + self.y*rhs.y
    }
}

impl Add for Point
{
    type Output = Point;
    fn add(self, rhs:Point) -> Point 
    {
        Point{
            x: self.x+rhs.x,
            y: self.y+rhs.y
        }
    }
}

impl Sub for Point
{
    type Output = Point;
    fn sub(self, rhs:Point) -> Point
    {
        Point{
            x: self.x-rhs.x,
            y: self.y-rhs.y
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub enum Shape {
    Point(Point),
    Segment(Point,Point),
    Arc{center:Point,radius:f64,start:f64,circ:f64},
}

fn norm_angle(ang:f64) -> (f64,f64)
{
    let tau = f64::consts::PI * 2.0;
    let rots = ang/tau;
    (rots.fract()*tau,rots.trunc())
}

fn angle_delta(start:f64, end: f64) -> f64
{
    match norm_angle(end - start) {
        (x,_) if x > f64::consts::PI => x - f64::consts::PI*2.0,
        (x,_) => x
    }
}

impl Shape {
    pub fn param(&self, t: f64) -> Point
    {
        match self {
            &Shape::Point(point) => point,
            &Shape::Segment(a,b) => Point{
                x: b.x*t + (1.0-t)*a.x,
                y: b.y*t + (1.0-t)*a.y
            },
            &Shape::Arc{center,radius,start,circ} => {
                Point {
                    x: center.x + radius*f64::cos(start + t*circ),
                    y: center.y + radius*f64::sin(start + t*circ),
                }
            }
        }
    }
    pub fn nearpoints(&self, point: Point) -> Vec<(f64,f64)> // (distance,param)
    {
        match self {
            &Shape::Point(p) => {
                vec![(p.dist(point),0.0)]
            },
            &Shape::Segment(a,b) => {
                let rel = point - a;
                let x = (b-a) * rel;
                let t = match x {
                    x if x > 1.0 => 1.0,
                    x if x < 0.0 => 0.0,
                    x => x
                };
                let d = self.param(t).dist(point);
                vec![(d,t)]
            },
            &Shape::Arc{center,radius,start,circ} => {
                let delta = point - center;
                let theta = delta.y.atan2(delta.x);
                let mut out = Vec::new();
                if (angle_delta(theta,start) > 0.0) == (circ > 0.0) {
                    out.push((self.param(0.0).dist(point),0.0));
                }
                if (angle_delta(theta,start+circ) > 0.0) != (circ > 0.0) {
                    out.push((self.param(1.0).dist(point),1.0));
                }
                out
            }
        }
    }
    pub fn intersect(&self, other: &Shape) -> Vec<Shape>
    {
        match self {
            &Shape::Point(self_p) => {
                match other {
                    &Shape::Point(other_p) => {
                        if other_p.x == self_p.x && other_p.y == self_p.y {
                            vec![self.clone()]
                        } else {
                            Vec::new()
                        }
                    },
                    _ => other.nearpoints(self_p).iter().filter_map(|&(d,t)| {
                        if d == 0.0 {
                            Some(Shape::Point(other.param(t)))
                        } else {
                            None
                        }
                    }).collect()
                }
            },
            _ => Vec::new()
        }
    }
}


