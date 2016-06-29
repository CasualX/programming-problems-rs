// Write a function that merges two sorted lists into a new list.

fn main() {
    let a_vec: Vec<i32> = vec![5, 6, 7];
    let b_vec: Vec<i32> = vec![1, 2, 3, 4];
 
    let c_vec = merge_sorted::<i32>(&a_vec, &b_vec);
 
    println!("{:?} ~ {:?} => {:?}", a_vec, b_vec, c_vec);
}
 
fn merge_sorted<T: Clone + Ord>(x: &[T], y: &[T]) -> Vec<T> {
    let mut merged: Vec<T> = vec![x[0].clone(); x.len()];
 
    merged.clone_from_slice(x);
    merged.extend_from_slice(y);
 
    merged.sort();
    merged
}