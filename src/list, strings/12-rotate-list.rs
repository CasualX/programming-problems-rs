// Write a function that rotates a list by k elements. For example [1,2,3,4,5,6] rotated by two becomes [3,4,5,6,1,2].
// Try solving this without creating a copy of the list. How many swap or move operations do you need?

fn main() {
	let mut list = [1, 2, 3, 4, 5, 6];
	println!("Input:   {:?}", list);
	rotate_inplace(&mut list, 2);
	println!("Rotated: {:?}", list);
}

// Naive rotate must be done one place at the time.
// https://stackoverflow.com/questions/4457277/algorithm-to-rotate-an-array-in-linear-time

fn rotate_inplace<T>(list: &mut [T], k: usize) {
	list[..].reverse();
	list[..k].reverse();
	list[k..].reverse();
}
