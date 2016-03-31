// Write a program that asks the user for a number n and prints the sum of the numbers 1 to n.

use std::io::{self, Write};

fn main() {
	print!("Enter a positive integer: ");
	io::stdout().flush().unwrap();

	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();

	// Shadow the local variable `n` by its read-only integer representation
	let n: u32 = n.trim().parse().unwrap();

	// Sum the integers using basic loop
	let mut sum = 0u32;
	for i in 1..n + 1 {
		sum += i;
	}

	println!("The sum of integers in range [1, {}] is {}.", n, sum);
}
