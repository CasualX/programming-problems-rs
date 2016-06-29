// Write a function that returns the largest element in a list.

fn main() {
	let mut list = [1, 2, 3, 4, 5];
	println!("{:?}", list);
	reverse_list(&mut list);
	println!("{:?}", list);
}

fn reverse_list<T: Copy>(list: &mut [T]) {
	// let len = list.len();
	// for i in 0..len / 2 {
	// 	list.swap(i, len - i - 1);
	// }
	list.reverse() // Does this reverse INPLACE?
}