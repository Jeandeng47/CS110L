use std::fmt;

// Generic T: declare type when creating
// let pair = MatchingPair::new(1, 2); // MatchingPair<i32>
// let pair = MatchingPair::new('a', 'b'); // MatchingPair<char>

pub struct MatchingPair<T> {
    first: T,
    second: T
}

// custom enum: hold some value of T, or nothing
// let some_value = MyOption::Sumthin(42); // MyOption<u32>
// let no_value = MyOption::Nuthin;       // MyOption<u32>

pub enum MyOption<T> {
    Sumthin(T),
    Nuthin
}

impl<T> MatchingPair<T> {
    pub fn new(first: T, second: T) -> Self {
        MatchingPair { first : first, second : second}
    }
}

// impl fmt::Display for MatchingPair<char> {
//     fn fmt(&self, f: &mut fmd::Formatter<'_>) -> fmt::Result {
//         write!(f, "({} {})", self.first, self.second)
//     }
// }

// General:  
// "T: fmt::Display" : impl only valid if T impls Display

impl<T: fmt::Display> fmt::Display for MatchingPair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.first, self.second)
    }
}


// impl fmt::Display for MyOption<u32> {
//     fn fmt(&self, f: &mut fmd::Formatter<'_>) -> fmt::Result {
//         match self {
//             MyOption::Sumthin(num) => write!(f, "Sumthin({})", num),
//             MyOption::Nuthin => write!(f, "Nuthin :("),
//         }
//     }
// }

// General:
impl<T: fmt::Display> fmt::Display for MyOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyOption::Sumthin(value) => write!(f, "Sumthin({})", value),
            MyOption::Nuthin => write!(f, "Nuthin :("),
        }
    }
}

fn main() {
    let char_pair = MatchingPair::new('x', 'y');
    let int_pair = MatchingPair::new(10, 20);
    let string_pair = MatchingPair::new("Hello", "World");

    println!("{}", char_pair);   // (x y)
    println!("{}", int_pair);    // (10 20)
    println!("{}", string_pair); // (Hello World)

    let some_number = MyOption::Sumthin(42);
    let some_string = MyOption::Sumthin("Hello");
    let none: MyOption<u32> = MyOption::Nuthin;

    println!("{}", some_number);  // Sumthin(42)
    println!("{}", some_string);  // Sumthin(Hello)
    println!("{}", none);         // Nuthin :(


}
