// Modify the previous program such that only multiples of three or five are considered in the sum, e.g. 3, 5, 6, 9, 10, 12, 15 for n=17.

use std::io::{self, Write};

fn main() {
	print!("Enter a positive integer: ");
	io::stdout().flush().unwrap();

	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();

	// Shadow the local variable `n` by its read-only integer representation
	let n: u32 = n.trim().parse().unwrap();

	// Sum the integers using iterators
	let sum = (1..n + 1)
		.filter(|i| i % 3 == 0 || i % 5 == 0)
		.fold(0, |acc, i| acc + i);

	println!("The sum of integers in range [1, {}] that are a multiple of 3 or 5 is {}.", n, sum);
}
