// Modify the previous program such that only the users Alice and Bob are greeted with their names.

use std::io::{self, Write};

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

	// Only greet Alice and Bob
	if name == "Alice" || name == "Bob" {
		println!("Greetings, {}!", name);
	} else {
		println!("I don't know anyone by that name.");
	}
}
