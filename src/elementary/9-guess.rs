// Write a guessing game where the user has to guess a secret number. 
// After every guess the program tells the user whether their number was too large or too small. 
// At the end the number of tries needed should be printed. 
// I counts only as one try if they input the same number multiple times consecutively.

extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
	println!("Guess the number between 1 and 100!");

	let secret_number = rand::thread_rng().gen_range(1,101);
	let mut guesses: u32 = 0;
	let mut prev_guess: u32 = 0;

	loop {
		println!("Please input your guess:");

		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input).expect("Failed to read line");

		let user_input: u32 = match user_input.trim().parse() {
			Ok(n) 	=> n,
			Err(_) 	=> { println!("Couldn't parse your input, try again."); continue; },
		};

		println!("You guessed {}", user_input);

		match user_input.cmp(&prev_guess) {
			Ordering::Equal => {},
			_				=> guesses += 1,
		};

		prev_guess = user_input;
		
		match user_input.cmp(&secret_number) {
			Ordering::Less 		=> println!("Too small!"),
			Ordering::Greater 	=> println!("Too big!"),
			Ordering::Equal 	=> { println!("You win"); break; },
		}
	}
	println!("Guessed {} times.", guesses);
}