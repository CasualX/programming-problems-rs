// Write a function that takes a list of strings an prints them, one per line, in a rectangular frame.
// For example the list ["Hello", "World", "in", "a", "frame"] gets printed as:

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

use std::cmp::max;

fn main() {
	frame("Hello World in a frame.");
}

// Extended functionality
fn frame(s: &str) {
	// Figure out width of the frame
	let width = s.unicode_words().fold(0usize, |acc, word| max(acc, word.len()));

	// Print header
	for _ in 0..width + 4 {
		print!("*");
	}
	println!("");

	for word in s.unicode_words() {
		println!("* {word:<width$} *", word = word, width = width);
	}

	// Print footer
	for _ in 0..width + 4 {
		print!("*");
	}
	println!("");
}
