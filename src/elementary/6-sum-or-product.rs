// Write a program that asks the user for a number n and gives him the possibility to choose between computing the sum and computing the product of 1,…,n.

use std::io::{self, Write};

// The choice between sum or product is represented with this enum
enum Op {
	Sum,
	Product,
}

fn main() {
	print!("Enter a positive integer: ");
	io::stdout().flush().unwrap();

	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();

	let n: u32 = n.trim().parse().unwrap();

	print!("Sum (+) or product (*)? ");
	io::stdout().flush().unwrap();

	let mut op = String::new();
	io::stdin().read_line(&mut op).unwrap();

	// Interpret the operator string by translating to the operator enum
	let op = match op.trim() {
		"+" => Op::Sum,
		"*" => Op::Product,
		_ => panic!("Invalid operator"),
	};

	// Calculate the result in two separate loops
	let result = match op {
		Op::Sum => {
			(1..n + 1).fold(0, |acc, x| acc + x)
		},
		Op::Product => {
			(1..n + 1).fold(1, |acc, x| acc * x)
		},
	};

	println!("The {} of integers in range [1, {}] is {}.",
		match op {
			Op::Sum => "sum",
			Op::Product => "product",
		},
		n,
		result);
}
