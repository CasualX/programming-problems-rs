// Write a function on_all that applies a function to every element of a list. Use it to print the first twenty perfect squares.

fn main() {
	on_all(1..20 + 1, |&x| {
		println!("{}", x * x)
	});
}

// Takes a generic Iterator and Fn
fn on_all<I, F>(it: I, f: F) where I: Iterator, F: Fn(&I::Item) {
	// Consume the iterator and call callable f on every iteration
	for x in it {
		f(&x)
	}
}
