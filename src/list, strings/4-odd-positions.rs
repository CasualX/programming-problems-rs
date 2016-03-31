// Write a function that returns the elements on odd positions in a list.

fn main() {
	let values = [5, 2, 8, -5, -1];
	let odds = odd_positions(&values);

	println!("Odd positions: {:?}", odds);
}

fn odd_positions<T: Copy>(list: &[T]) -> Vec<T> {
	let mut flip = false;
	list.iter().filter(|_| {
		// Flip flop skipping every other value
		flip = !flip;
		flip
	}).map(|&x| {
		// Strip off the reference
		x
	}).collect::<Vec<_>>()
}
