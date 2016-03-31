// Write a function that checks whether an element occurs in a list.

fn main() {
	let list = [5, 2, 8, -1];
	println!("Has {:?} element 2? {}", list, has_element(&list, &2));
	println!("Has {:?} element -3? {}", list, has_element(&list, &-3));
}

fn has_element<T: Eq>(list: &[T], value: &T) -> bool {
	list.iter().any(|it| *it == *value)
}
