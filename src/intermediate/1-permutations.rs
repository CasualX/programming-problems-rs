// Write a program that outputs all possibilities to put + or - or nothing between the numbers 1,2,…,9 (in this order) such that the result is 100.
// For example 1 + 2 + 3 - 4 + 5 + 6 + 78 + 9 = 100.

fn main() {
	compute("1".to_owned(), 0, 1, 2);
}

fn compute(s: String, total: i32, current: i32, digit: i32) {
	// Recurse end condition
	if digit == 10 {
		let total = total + current;
		if total == 100 {
			println!("found one: {} = 100!", s);
		}
	}
	else {
		// Concatenate the digit
		{
			let current = current * 10 + if current < 0 { -digit } else { digit };
			let s = format!("{}{}", s, digit);
			compute(s, total, current, digit + 1);
		}
		// Add the digit
		{
			let total = total + current;
			let s = format!("{} + {}", s, digit);
			compute(s, total, digit, digit + 1);
		}
		// Subtract the digit
		{
			let total = total + current;
			let s = format!("{} - {}", s, digit);
			compute(s, total, -digit, digit + 1);
		}
	}
}
