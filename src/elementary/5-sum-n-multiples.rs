// Modify the previous program such that only multiples of three or five are considered in the sum, e.g. 3, 5, 6, 9, 10, 12, 15 for n=17.

use std::io::{self, Write};

fn calc_sum(num: u32) -> u32 {
    let mod_filter = |&n: &u32| -> bool { n % 3 == 0 || n % 5 == 0 };
	(1..num + 1).filter(mod_filter).fold(0, |acc, x| acc + x)
}

fn main() {
	println!("Calculates the sum of integers, that are multiples of 3 or 5, from 1 to n");

	let number: u32;

	loop {
		println!("Enter a positive integer (n): ");
		io::stdout().flush().unwrap();

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
