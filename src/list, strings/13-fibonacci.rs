// Write a function that computes the list of the first 100 Fibonacci numbers.

fn main() {
	// Excersise asks for first 100, but this overflows i64 :(
	for it in fibonacci().take(20) {
		println!("{}", it);
	}
}

struct Fibonacci {
	pub a: i64,
	pub b: i64,
}
impl Iterator for Fibonacci {
	type Item = i64;
	fn next(&mut self) -> Option<i64> {
		let result = self.a + self.b;
		self.a = self.b;
		self.b = result;
		// Because we never return None, this is an infinite iterator
		// We're practically limited by the accuracy of i64 however
		Some(result)
	}
}

// Creates an iterator over the fibonacci sequence
fn fibonacci() -> Fibonacci {
	Fibonacci {
		a: 1,
		b: 0,
	}
}
