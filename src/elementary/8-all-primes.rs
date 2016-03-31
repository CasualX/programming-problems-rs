// Write a program that prints all prime numbers.
// (Note: if your programming language does not support arbitrary size numbers,
//  printing all primes up to the largest number you can easily represent is fine too.)

fn is_prime(n: i32) -> bool {
	// Fail fast for special case 2
	if n == 2 {
		return true;
	}
	if n % 2 == 0 {
		return false;
	}
	// Find highest number to check
	let end = (n as f64).sqrt().ceil() as i32;
	// Iterate over odd numbers starting from 3
	let mut i = 3;
	while i <= end {
		// Check if `n` is a composite
		if n % i == 0 {
			return false;
		}
		i += 2;
	}
	// Is prime
	return true;
}

fn main() {
	// Limit to primes smaller than 1000
	for i in (2..1000).filter_map(|i| if is_prime(i) { Some(i) } else { None }) {
		println!("{}", i);
	}
}
