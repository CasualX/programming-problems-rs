// Write a program that asks the user for a number n and gives him the possibility to choose between computing the sum and computing the product of 1,…,n.

use std::io::{self, Write};

fn calc_sum(num: u64) -> u64 {

	(1..num + 1).fold(0, |acc, x| acc + x)
}

fn calc_product(num: u64) -> u64 {

	(1..num + 1).fold(1, |acc, x| acc * x)
}

fn main() {
	println!("Calculates the sum or product of integers from 1 to n.");

	let mut number: u64;

	loop {
		print!("Sum (+) or product (*)? ");
		io::stdout().flush().unwrap();

		let mut input_choice = String::new();

		io::stdin().read_line(&mut input_choice).expect("Failed to read line.");

		print!("Input a number (n): ");
		io::stdout().flush().unwrap();

		let mut input_s = String::new();

		io::stdin().read_line(&mut input_s).expect("Failed to read line.");

		match input_s.trim().parse() {
			Ok(n) => {
				number = n;
			}
			Err(_) => {
				println!("Natural numbers only, please.");
				continue;
			}
		};

		match input_choice.trim() {
			"*" => {
				let product: u64 = calc_product(number);
				println!("The product of integers from 1 to {} is {}", number, product);
				break;
			}
			"+" => {
				let sum: u64 = calc_sum(number);
				println!("The sum of integers from 1 to {} is {}", number, sum);
				break;
			}
			_ => {
				println!("Not a valid choice!");
				println!("Try: * for product, or + for sum");
				continue;
			}
		};
	}
}
