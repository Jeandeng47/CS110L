// // Example 2: Return a local varibale in a function
// fn drip_drop() -> &String {
//     let s = String::from("hello world!");
   
//     return &s;
// }

 // Fix 2: Directly return a string
 fn drip_drop() -> String {
    let s = String::from("hello world!");
    return s;
 }


fn main() {
    // // Example 1: Cannot create mutable reference
    // // while immutable references are still active

    // let mut s = String::from("hello"); // mutable string

    // // ref1-ref3: all immutable refs
    // let ref1 = &s;
    // let ref2 = &ref1;
    // let ref3 = &ref2;

    // // ERROR: mutable borrow of `s`
    // s = String::from("goodbye");
    // println!("{}", ref3.to_uppercase());

    // Fix 1: Use scope to limit borrow life time
    let mut s = String::from("hello");

    {
        let ref1 = &s;
        let ref2 = &ref1;
        let ref3 = &ref2;
        println!("{}", ref3.to_uppercase());
    } // All immutable references go out of scope here

    s = String::from("goodbye"); // Mutable borrow is now allowed
    println!("{}", s);


    // Example 2: Return a local varibale in a function
    // drip_drop();

    let result = drip_drop();
    println!("{}", result);

    // Example 3:
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1); // ownership move to v
    // Error: v[0] returns a reference but not the String itself
    // let s2: String = v[0];
    // Fix 3: let s2 borrow from s1
    let s2:&String = &v[0];
    
    println!("{}", s2);

}
