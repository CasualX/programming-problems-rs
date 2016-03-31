// Write a function that returns the largest element in a list.

fn main() {
	let values = [5, 2, 8, -1];
	println!("Largest in {:?} is {:?}.", values, largest(&values));
}

fn largest<T: Ord>(list: &[T]) -> Option<&T> {
	// Essentially folding over the list, taking the max value
	// Needs special care for empty lists (hence the Option)
	list.iter().fold(None, |acc, x| {
		if let Some(acc) = acc {
			if x > acc {
				Some(x)
			}
			else {
				Some(acc)
			}
		}
		else {
			Some(x)
		}
	})
}
