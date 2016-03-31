// Write a function that computes the running total of a list.

fn main() {
	let values = [5, 2, 8, -5, -1];
	let sum = running_total(&values);
	println!("Running total of {:?} is {}.", values, sum);
}

// Rust 1.7; support for working generically over numeric types isn't quite there yet
fn running_total(list: &[i32]) -> i32 {
	list.iter().fold(0, |acc, x| {
		acc + x
	})
}
