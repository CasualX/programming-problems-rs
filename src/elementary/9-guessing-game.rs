// Write a guessing game where the user has to guess a secret number.
// After every guess the program tells the user whether their number was too large or too small.
// At the end the number of tries needed should be printed.
// It counts only as one try if they input the same number multiple times consecutively.

extern crate rand;

use std::io::{self, Write};
use std::cmp::Ordering;

use rand::distributions::IndependentSample;

fn main() {
	println!("Welcome to the guessing game!");

	// Difficulty
	let range = 1..100 + 1;
	let max_tries = ((range.end - range.start) as f64).log2().ceil() as i32;
	println!("Guess in range [{}, {}].", range.start, range.end - 1);

	// Generate the secret random number
	let secret = rand::distributions::Range::new(range.start, range.end).ind_sample(&mut rand::thread_rng());

	// Infinite loop until you get it right
	// If you must, ctrl-C quits
	let mut tries = 1;
	let mut last_guess = -1;
	loop {
		print!("> ");
		io::stdout().flush().unwrap();

		let mut guess = String::new();
		io::stdin().read_line(&mut guess).unwrap();

		// Validate parse and validate the guess
		let guess = match guess.trim().parse::<i32>() {
			Ok(n) => {
				if n < range.start || n >= range.end {
					println!("Guess in range [{}, {}]!", range.start, range.end - 1);
					continue;
				}
				else {
					n
				}
			},
			Err(_) => {
				println!("Not a number!");
				continue;
			},
		};

		// Stop entering the same number twice
		if guess == last_guess {
			println!("You just tried that already!");
			continue;
		}

		// Compare to secret if you guessed it
		match secret.cmp(&guess) {
			Ordering::Equal => {
				break;
			},
			Ordering::Less => {
				println!("Less!");
			},
			Ordering::Greater => {
				println!("Greater!");
			},
		}

		last_guess = guess;
		tries += 1;
	}

	// Game finished, you're a winner!
	println!("Congratulations!");

	let score = max_tries - tries;
	println!("You took {} tries, score: {}.", tries, score);

	// Opinionated rating
	println!("Rating: {}!",
		if score >= 0 {
			if score >= max_tries / 2 { "Lucky" }
			else { "Professional" }
		}
		else {
			if score >= -max_tries / 2 { "Amateur" }
			else { "Noob" }
		});
}
