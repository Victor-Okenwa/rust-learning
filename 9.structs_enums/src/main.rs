struct Point {
    x: i32,
    y: i32,
}

// we can add methods to structs using impl keyword
impl Point {
    // associated function (no self) — like a static method
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    // method (takes &self) — like a class method
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

// enum 
enum Shape {
    Circle (f64),
    Rectangle(f64, f64), // tuple struct
    Triangle {base: f64, height: f64}, // struct with named fields
    Empty, // unit struct (no fields)
}

fn area  (shape: &Shape) -> f64 {
    match shape {
        Shape
    ::Circle(radius) => 3.14 * radius * radius,
        Shape
    ::Rectangle(width, height) => width * height,
        Shape
    ::Triangle {base, height} => 0.5 * base * height,
        Shape
    ::Empty => 0.0,
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point: ({}, {})", p.x, p.y);

    let o = Point::origin();
    println!("Origin: ({}, {})", o.x, o.y);

    println!("Distance from origin: {}", p.distance_from_origin());

    let c = Shape
::Circle (10.0);
println!("Area of circle: {}", area(&c));

    let r = Shape::Rectangle(10.0, 20.0);
    println!("Area of rectangle: {}", area(&r));

    let t = Shape::Triangle { base: 10.0, height: 20.0 };
    println!("Area of triangle: {}", area(&t));

    let e = Shape::Empty;
    println!("Area of empty: {}", area(&e));

}
