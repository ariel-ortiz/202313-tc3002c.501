fn main() {
    let sh1 = Square::new(2.5);
    let sh2 = Triangle::new(4.5, 2.0);
    let sh3 = Circle::new(3.0);
    let sh4 = sh1; // copy
    let sh5 = sh3.clone();
    shape_report(&sh1);
    shape_report(&42);
    shape_report(&sh2);
    shape_report(&sh3);
}

fn shape_report<T: Shape>(shape: &T) {
    println!("SHAPE: {}", shape.description());
    println!("-------------------------------------------");
    println!("Area = {:.2} m^2", shape.area());
    println!("Perimeter = {:.2} m", shape.perimeter());
    println!();
}

trait Shape {

    fn area(&self) -> f64;

    fn perimeter(&self) -> f64;

    fn description(&self) -> String {
        "unknown".to_string()
    }
}

#[derive(Copy, Clone)]
struct Square {
    side: f64
}

impl Square {

    fn new(side: f64) -> Square {
        Square {
            side
        }
    }
}

impl Shape for Square {

    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }

    fn description(&self) -> String {
        format!("Square (side = {} m)", self.side)
    }
}

impl Shape for i32 {

    fn area(&self) -> f64 {
        f64::NAN
    }

    fn perimeter(&self) -> f64 {
        f64::NAN
    }

    fn description(&self) -> String {
        "i32".to_string()
    }
}

struct Triangle {
    base: f64,
    height: f64
}

impl Triangle {

    fn new(base: f64, height: f64) -> Triangle {
        Triangle { base, height }
    }
}

impl Shape for Triangle {

    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }

    fn perimeter(&self) -> f64 {
        self.base * 3.0
    }

    fn description(&self) -> String {
        format!("Triangle (base = {} m, height = {} m)",
            self.base, self.height)
    }
}

struct Circle {
    radius: f64
}

impl Circle {

    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Shape for Circle {

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        std::f64::consts::PI * self.radius * 2.0
    }

    fn description(&self) -> String {
        format!("Circle (radius = {})", self.radius)
    }
}

impl Clone for Circle {

    fn clone(&self) -> Circle {
        println!("Me están clonando!!!");
        Circle { radius: self.radius.clone() }
    }
}
