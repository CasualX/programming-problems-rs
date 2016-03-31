// Write three functions that compute the sum of the numbers in a list: using a for-loop, a while-loop and recursion.
// (Subject to availability of these constructs in your language of choice.)

fn main() {
	let values = [5, 2, 8, -5, -1];

	println!("Input: {:?}", values);
	println!("Sum for-loop:   {}", sum_for(&values));
	println!("Sum while-loop: {}", sum_while(&values));
	println!("Sum recursion:  {}", sum_rec(0, &values));
}

fn sum_for(list: &[i32]) -> i32 {
	let mut acc = 0i32;
	for &it in list {
		acc += it;
	}
	return acc;
}
fn sum_while(list: &[i32]) -> i32 {
	let mut acc = 0i32;
	let mut it = list.iter();
	while let Some(&val) = it.next() {
		acc += val;
	}
	return acc;
}
fn sum_rec(val: i32, list: &[i32]) -> i32 {
	if let Some((&first, tail)) = list.split_first() {
		sum_rec(val + first, tail)
	}
	else {
		val
	}
}
