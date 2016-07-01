// Write a program that asks the user for a number n and prints the sum of the numbers 1 to n.

use std::io;

fn calc_sum(num: u32) -> u32 {
	(1..num + 1).fold(0, |acc, x| acc + x)
}

fn main() {
	println!("Calculates the sum of numbers from 1 to n, where n is user input.");
	let number: u32;

	loop {
		println!("Input a number:");
		let mut s_input = String::new();

		io::stdin().read_line(&mut s_input).expect("Failed to read line.");
			
		match s_input.trim().parse() {
			Ok(n) 	=> { number = n; break; },
			Err(_) 	=> { println!("A whole integer, please..."); continue; },
		};

	};
		let sum = calc_sum(number);
		println!("The sum of integers from 1 to {} is {}", number, sum);
}