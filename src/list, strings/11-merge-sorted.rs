// Write a function that merges two sorted lists into a new list.

fn main() {
	let left = [1, 5, 7];
	let right = [-2, 8, 9];

	let merged = merge_sorted(&left, &right);

	println!("Merge sorted {:?} and {:?} results {:?}.", left, right, merged);
}

fn merge_sorted<T: Copy + Ord>(left: &[T], right: &[T]) -> Vec<T> {
	let mut vec: Vec<T> = Vec::with_capacity(left.len() + right.len());

	// I couldn't find a way to compose this using the iterator building blocks, so back to basics it is!
	// This is also probably horribly inefficient, it definitely looks inelegant and fugly.

	let mut left_it = left.iter().peekable();
	let mut right_it = right.iter().peekable();

	loop {
		// Ok Borrowck complains that I cannot borrow left_it or right_it as mutable to call .next() on it while I'm still holding on to the peeked values
		// Idk how to solve this elegantly, so just slap this on it
		let side: bool; // false = left, true = right

		{
			let left_val = left_it.peek();
			let right_val = right_it.peek();

			if let Some(&left_val) = left_val {
				if let Some(&right_val) = right_val {
					vec.push(
						*if *left_val < *right_val {
							side = false;
							left_val
						}
						else {
							side = true;
							right_val
						});
				}
				else {
					side = false;
					vec.push(*left_val);
				}
			}
			else if let Some(&right_val) = right_val {
				side = true;
				vec.push(*right_val);
			}
			else {
				break;
			}
		}

		// Now all peeked borrows are gone and this will work
		if side {
			right_it.next();
		}
		else {
			left_it.next();
		}
	}
	vec
}
