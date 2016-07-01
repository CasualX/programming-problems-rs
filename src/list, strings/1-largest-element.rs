// Write a function that returns the largest element in a list.

fn largest_element<T: Ord>(list: &[T]) -> Option<&T> {
	let mut largest = None;
	for x in list {
		if let Some(n) = Some(x) {
			if Some(n) > largest {
				largest = Some(n)
			}
		}
	}
	largest
}

fn main() {
	let list = [-5, -7, 2, 0, -1, 7, 12, 6, 4, -10];
	println!("Largest element in list: {:?} is {:?}", list, largest_element(&list));
	let empty_list: Vec<i32> = Vec::new();
	println!("Largest element in list: {:?} is {:?}", empty_list, largest_element(&empty_list));
}
