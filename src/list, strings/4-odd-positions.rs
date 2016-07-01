// Write a function that returns the elements on odd positions in a list.

fn main() {
	let values = [1,2,3,4,5,6,7,8,9,10];
	let odds = odd_positions(&values);

	println!("Odd positions: {:?}", odds);
}

fn odd_positions<T: Copy>(list: &[T]) -> Vec<T> {
	let mut odds_vec = Vec::new();
	for (i, &value) in list.iter().enumerate() {
		if i % 2 != 0 {
			odds_vec.push(value);
		}
	}
	odds_vec
}
