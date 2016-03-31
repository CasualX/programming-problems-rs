// Write a program that prints a multiplication table for numbers up to 12.

fn main() {
	let n = 12 + 1;

	// Start by printing the header
	print!("  x |");
	for i in 2..n {
		print!("{:>4} |", i);
	}
	println!("");

	// Print the header separator
	print!("----+");
	for _ in 2..n {
		print!("-----+");
	}
	println!("");

	// Print each row
	for row in 2..n {
		print!("{:>3} |", row);

		// Print each entry
		for col in 2..n {
			let e = row * col;
			print!("{:>4} |", e);
		}

		println!("");
	}
}
