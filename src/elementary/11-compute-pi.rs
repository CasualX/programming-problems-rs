// Write a program that computes PI.

use std::f64::consts::PI;

fn main() {
	// What is good formatting for this statement
	let result = (1..1000000)
		.map(|k| {
			let num = if k & 1 != 0 { 1f64 } else { -1f64 };
			let denom = ((k + k) - 1) as f64;
			num / denom
		})
		.fold(0f64, |acc, x| acc + x) * 4f64;

	println!("Result is {}.", result);
	println!("Error is  {}.", result - PI);
}
