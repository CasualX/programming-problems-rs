// Write function that reverses a list, preferably in place.

fn main() {
	let mut list = [5, 2, 8, -1];
	println!("The input list {:?}.", list);
	reverse_inplace(&mut list);
	println!("Reversed input {:?}.", list);
}

fn reverse_inplace<T: Copy>(list: &mut [T]) {
	// Pretty dumb but straightforward mutation
	let len = list.len();
	for i in 0..len / 2 {
		list.swap(i, len - i - 1);
	}
}
