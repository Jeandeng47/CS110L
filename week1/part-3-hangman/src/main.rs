// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand; // library for random number generation
use rand::Rng; // trait for methods that random number generators implement
use std::fs; // library for working with the filesystem
use std::io; // library for user input
use std::io::Write; // trait for flushing the output buffer

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].

    let secret_word = pick_a_random_word();
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    println!("random word: {}", secret_word);

    println!("Welcome to CS110L Hangman!");
    
    let len = secret_word.len();
    let mut guessed_word: Vec<char> = vec!['-'; len];
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut guess_count = 0;

    while guess_count < NUM_INCORRECT_GUESSES {
        println!("The word so far is: {}", guessed_word.iter().collect::<String>());
        println!("You have guessed the following letters: {}", guessed_letters.iter().collect::<String>()); 
        println!("You have {} guesses left", NUM_INCORRECT_GUESSES - guess_count);

        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line.");
        

        // Extract the first character ()
        // .next(): returns an Option<char>
        let guess_char = guess.trim().chars().next();

        // Check if the char is none (unwrap Option<char>)
        if let Some(c) = guess_char {
            // Check if the char is guessed
            if guessed_letters.contains(&c) {
                println!("You have gussed the letter before!");
            }
            guessed_letters.push(c);

            // Check if the char is in the secret word
            if secret_word_chars.contains(&c) {
                // If match, reveal all that chars in the word
                for (i, &char) in secret_word_chars.iter().enumerate() {
                    if char == c {
                        guessed_word[i] = c;
                    }
                }

                // Check if the word is guessed
                if !guessed_word.contains(&'-') {
                    println!("Congratulations! You guessed the word: {}", secret_word);
                    return;
                }
            } else {
                println!("Sorry, that letter is not in the word!");
                guess_count += 1;
            }
        } else {
            println!("Please enter a valid character!");
        }
        println!();     
    }

    println!("Sorry, you ran out of guesses! The word was: {}", secret_word);
}
