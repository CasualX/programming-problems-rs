// Write a function that takes a number and returns a list of its digits.

fn main() {
	let n = 123456789u64;
	print!("digits({}): ", n);

	let num = digits(&n);
	for d in num {
		print!("{} ", d);
	}
	println!("");
}

fn digits(n: &u64) -> Vec<u8> {
	let mut vec: Vec<u8> = Vec::new();

	for i in n.to_string().chars() { // Convert digits to individual chars of a string
		vec.push(i.to_digit(10).unwrap() as u8); // Push each char to vec as u8 digit (in base 10)
	}

	vec
}