// Write a function that takes a list of strings an prints them, one per line, in a rectangular frame. For example the list ["Hello", "World", "in", "a", "frame"] gets printed as:

// *********
// * Hello *
// * World *
// * in    *
// * a     *
// * frame *
// *********

use std::cmp::max;

fn main() {
	frame("Hello world, print this in a frame!");
}

// Extended functionality
fn frame(s: &str) {
	// Figure out width of the frame
	let width = s.split_whitespace().fold(0usize, |acc, word| max(acc, word.len()));

	// Print header
	for _ in 0..width + 4 {
		print!("*");
	}
	println!("");

	for word in s.split_whitespace() {
		println!("* {word:<width$} *", word = word, width = width);
	}

	// Print footer
	for _ in 0..width + 4 {
		print!("*");
	}
	println!("");
}