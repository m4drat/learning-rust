use std::f32;
use std::fmt::Display;

#[derive(Debug)]
struct Point2D {
    x: f32,
    y: f32,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x = {}, y = {}", self.x, self.y)
    }
}

impl Point2D {
    fn distance_to_origin(&self, origin: Option<&Point2D>) -> f32 {
        let origin = origin.unwrap_or(&Point2D { x: 0.0, y: 0.0 });

        f32::sqrt(f32::powf(origin.x - self.x, 2.) + f32::powf(origin.y - self.y, 2.))
    }
}

fn main() {
    let point = Point2D { x: 1., y: 1. };

    println!("Implemented display: {}", point);
    println!("Default dbg print: {:#?}", point);
    println!("Distance to origin: {:#?}", point.distance_to_origin(None));
}
