// Write a function that concatenates two lists.

fn main() {
    let a_vec: Vec<i32> = vec![1, 2, 3, 4];
    let b_vec: Vec<i32> = vec![5, 6, 7];

    let c_vec = concatenate_arrays::<i32>(&a_vec, &b_vec);

    println!("{:?} ~ {:?} => {:?}", a_vec, b_vec, c_vec);
}

fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat: Vec<T> = vec![x[0].clone(); x.len()];

    concat.clone_from_slice(x);
    concat.extend_from_slice(y);

    concat
}
