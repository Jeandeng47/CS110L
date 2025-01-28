use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead}; // For read_file_lines()

// Implement the most basic form of the wc command – 
// given an input file, output the number of words, lines, 
// and characters in the file

fn read_file_lines(filename : &String) -> Result<Vec<String>, io::Error> {

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let lines : Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let lines = read_file_lines(filename).expect("Could not read from file.");

    let num_lines = lines.len();
    let num_words : usize = lines
        .iter()
        .map(|line| {
            let words_in_line = line.split_whitespace();
            let words_count = words_in_line.count();
            words_count
        })
        .sum();


    let num_chars : usize = lines
        .iter()
        .map(|line| {
            let chars_count = line.chars().count();
            chars_count
        })
        .sum();

    println!("Number of lines: {}", num_lines);
    println!("Number of words: {}", num_words);
    println!("Number of characters: {}", num_chars);
}
