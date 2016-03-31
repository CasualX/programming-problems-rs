// Write a function that takes a number and returns a list of its digits.

use std::char;

fn main() {
	let n = 123456789u64;
	print!("digits({}): ", n);

	let num = digits(n);
	for d in num.iter().rev().map(|x| char::from_u32(*x as u32 + 48).unwrap()) {
		print!("{:?} ", d);
	}
	println!("");
}

fn digits(mut n: u64) -> Vec<u8> {
	let mut vec: Vec<u8> = Vec::new();

	while n != 0 {
		let r = (n % 10) as u8;
		n = n / 10;
		vec.push(r);
	}

	if vec.len() == 0 {
		vec.push(0);
	}

	vec
}
