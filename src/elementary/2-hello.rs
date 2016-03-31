// Write a program that asks the user for her name and greets her with her name.

use std::io;
use std::io::Write;

fn main() {
	print!("Enter your name: ");

	// Make sure the written input is flushed to the console before writing new stuff...
	// [`print!` macro should flush stdout #23818](https://github.com/rust-lang/rust/issues/23818)
	io::stdout().flush().unwrap();

	// Read a line from standard input to `name`
	let mut name = String::new();
	io::stdin().read_line(&mut name).unwrap();

	// Strip the trailing newline we received from stdin
	let name = name.trim();

	println!("Greetings, {}!", name);
}
