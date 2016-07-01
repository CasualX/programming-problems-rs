// Write a function that combines two lists by alternatingly taking elements, e.g. [a,b,c], [1,2,3] → [a,1,b,2,c,3].

fn main() {
	let left = [1, 2, 3];
	let right = [10, 20, 30];

	let alt = alternating(&left, &right);

	println!("Interleaved {:?} and {:?} results {:?}.", left, right, alt);
}

fn alternating<T: Copy>(left: &[T], right: &[T]) -> Vec<T> {
	// Prealloc because we're nice like that :)
	let mut vec: Vec<T> = Vec::with_capacity(left.len() + right.len());
	// Zip creates a pair-wise iterator over both iterators using UFCS
	for (&l, &r) in left.into_iter().zip(right) {
		vec.push(l);
		vec.push(r);
	}
	vec
}
