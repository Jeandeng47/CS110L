/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    // takes ownership of v
    // .iter(): iterates over the vector
    // .map(): applies a function to each element
    // .collect(): collects the results into a new vector
    // |x|: lambda function that takes x as an argument

    v.iter().map(|x| x + n).collect()
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    // takes mutable reference to v
    // .iter_mut(): iterates over the vector mutably
    // *x : deferences x to modify the value

    for x in v.iter_mut() {
        *x += n;
    }

}

fn dedup(v: &mut Vec<i32>) {
    // hashset: remove duplicates IN-PLACE
    // .retain(): retains elements if the closure returns true
    // seen.insert(*x): HashSet::insert returns false if the value is already in the set
    // v.retain(): retains elements if the closure returns true

    let mut seen = HashSet::new();
    v.retain(|x| seen.insert(*x));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
