// Write a function that checks whether an element occurs in a list.

fn main() {
	let value = 4;
	let list = [3, 2, 1, -5, 6, 4];
	println!("It is {} that list {:?} has value \"{}\" in it", has_element(&list, &value), list, value);
}

fn has_element<T: Eq>(list: &[T], value: &T) -> bool {
	list.iter().any(|x| x == value)
}