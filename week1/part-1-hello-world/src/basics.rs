/*
    n: 1
    Inferred m: 1
    Mutable x: 2
    &str: Hello, world!
    String: Hello, world! Welcome to Rustland!
    Vector: [1, 2, 3]
    Array: [-4, 5, 6]
    First element: -4
    Vector element: 1
    Vector element: 2
    Vector element: 3
    Counter: 0
    Counter: 1
    Counter: 2
    Tick: 0
    Tick: 1
    Tick: 2
    Sum: 3
    This is a Rust function
*/

fn main() {
    // Declare with / without type
    let n : i32 = 1;
    println!("n: {}", n);

    let m = 1;
    println!("Inferred m: {}", m);

    // Mutable variable
    let mut x = 1;
    x = x + 1;
    println!("Mutable x: {}", x);

    // Difference between &str and String
    // &str: immutable pointer
    let s : &str = "Hello, world!";
    println!("&str: {}", s);

    // String: heap-allocated, mutable, freed after use
    let mut heap_s : String =  String::from("Hello, world!");
    heap_s.push_str(" Welcome to Rustland!");
    println!("String: {}", heap_s);

    // Vectors
    let mut v : Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v);

    // Arrays
    let mut arr : [i32; 3] = [4, 5, 6];
    arr[0] = -4;
    println!("Array: {:?}", arr);
    println!("First element: {}", arr[0]);

    // Iterating over vector
    for i in v.iter() {
        println!("Vector element: {}", i);
    }

    // While loop
    let mut counter = 0;
    while counter < 3 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    // Loop with break
    let mut tick = 0;
    loop {
        println!("Tick: {}", tick);
        tick += 1;
        if tick == 3 {
            break;
        }
    }

    // Function 
    fn sum(a: i32, b: i32) -> i32 {
        a + b // no return, no semi-colon
    }

    let result = sum(1, 2);
    println!("Sum: {}", result);

    fn print_msg(msg : &str) {
        println!("{}", msg);
    }

    print_msg("This is a Rust function");

    
}