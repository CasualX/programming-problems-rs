// Write a function that returns the largest element in a list.

use std::cmp::Ordering;

fn largest_element<T: Ord>(list: &[T]) -> &T {
	let mut largest = &list[0];
	for x in list {
		if let Ordering::Greater = x.cmp(&largest) {
			largest = x;
		}
	}
	largest
}

fn main() {
	let list = [-43, 23, -3, -12, 20, 3];
	println!("Largest element in list: {:?} is {}", list, largest_element(&list));
}