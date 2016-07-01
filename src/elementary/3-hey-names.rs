// Modify the previous program such that only the users Alice and Bob are greeted with their names.

use std::io::{self, Write};

fn main() {

	let mut name = String::new();

	println!("What's your name?");
	print!("My name is: ");

	io::stdout().flush().unwrap();

	io::stdin().read_line(&mut name).expect("Failed to read line.");
		
	let name = name.trim();

	match name {
		"Bob" 	=> println!("Hello, {}!", name),
		"Alice" => println!("Hello, {}!", name),
		_ 		=> println!("I don't know anyone with that name."),
	};
}
