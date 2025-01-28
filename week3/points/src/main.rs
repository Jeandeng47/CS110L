// #[derive(...)]: implement the traits inside
// Debug:  format when printing with use of {:?} and {:#?}
// PartialEq: compare the field for equality using != and ==
// Clone: create a copy of a instance (explicity)
// Copy:  allows the Point struct to be implicitly duplicated 
//
// fn main() {
//     let s1 = String::from("Hello, Rust!");

//     // Clone creates a deep copy of the string
//     let s2 = s1.clone();

//     println!("s1: {}", s1); // s1 is still valid
//     println!("s2: {}", s2); // s2 contains a new copy of the string

//     // Attempt to copy without clone (invalid)
//     let s3 = s1; // Moves `s1` to `s3`

//     // s1 is no longer valid here
//     println!("s2: {}", s3);
//     // println!("s1: {}", s1); // ERROR: borrow of moved value: `s1`
// }

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x : f64,
    y : f64,
}

impl Point {
    pub fn new(x : f64, y : f64) -> Self {
        Point{x : x, y : y}
    }
}

// Example: define our own trait
pub trait ComputeNorm {
    // default,  can be overridden when the 
    // trait is implemented for specific types
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for Option<u32> {}

impl ComputeNorm for Point {
    fn compute_norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl ComputeNorm for Vec<f64> {
    fn compute_norm(&self) -> f64 {
        self.iter().map(|x| {x * x}).sum::<f64>().sqrt()
    }
}

// Overload the + operator
impl Add for Point {
    type Output = Self; // Self: an associated type (Point here)
    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}


fn main() {
    // COPY trait
    let the_origin = Point::new(0.0 , 0.0);
    let mut p = the_origin;
    println!("p: {:?}, the_origin: {:?}", p, the_origin);
    println!("are they equal? => {}", p == the_origin); // true
    p.x += 10.0;
    println!("p: {:?}, the_origin: {:?}", p, the_origin);
    println!("are they equal? => {}", p == the_origin); // false

    // Custom trait ComputeNorm
    let p0 = Point::new(3.0, 4.0);
    println!("Norm of p0: {}", p0.compute_norm()); // Norm of p0: 5

    let v = vec![1.0, 2.0, 3.0];
    println!("Norm of vec: {}", v.compute_norm()); // Norm of vec: 3.7416573867739413

    // Overloaded add
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(3.0, 4.0);
    let p3 = p1 + p2;
    println!("Sum of points: {:?}", p3); // Sum of points: Point { x: 4.0, y: 6.0 }
}
