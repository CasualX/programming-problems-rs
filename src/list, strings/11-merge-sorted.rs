// Write a function that merges two sorted lists into a new list.

use std::cmp::Ordering;

fn main() {
    let a: Vec<i32> = vec![8, 10, 55, 60];
    let b: Vec<i32> = vec![-2, 16, 32, 114];

    let c = merge_sorted(&a, &b);

    println!("{:?} ~ {:?} => {:?}", a, b, c);
}

fn merge_sorted<T: Copy + Ord>(a: &[T], b: &[T]) -> Vec<T> {

    let mut i = 0;
    let mut j = 0;
    let mut vec: Vec<T> = Vec::with_capacity(a.len() + b.len());

    while i < a.len() || j < b.len() {
        match (a.get(i), b.get(j)) {
            (None, Some(&x)) => {
                vec.push(x);
                j += 1;
            }
            (Some(&x), None) => {
                vec.push(x);
                i += 1;
            }
            (Some(&x), Some(&y)) => {
                match x.cmp(&y) {
                    Ordering::Less => {
                        vec.push(x);
                        i += 1;
                    }
                    Ordering::Greater => {
                        vec.push(y);
                        j += 1;
                    }
                    Ordering::Equal => {
                        vec.push(y);
                        j += 1;
                    }
                }
            }
            (None, None) => unreachable!("Never get here"),
        }
    }
    vec
}
