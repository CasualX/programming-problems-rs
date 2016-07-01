// Implement the following sorting algorithms: Selection sort, Insertion sort, Merge sort, Quick sort, Stooge Sort. Check Wikipedia for descriptions.

extern crate rand;
use rand::Rng;

fn rand_vec(n: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity(12);
   for _ in 0..n {
        vec.push(rand::thread_rng().gen_range(0, 100));
    }
    vec
}

fn main() {
    let mut numbers = rand_vec(10);
    println!("Before: \t\t{:?}", numbers);
    selection_sort(&mut numbers);
    println!("Selection sorted: \t{:?}", numbers);

    let mut numbers = rand_vec(10);
    println!("Before: \t\t{:?}", numbers);
    insertion_sort(&mut numbers);
    println!("Insertion sorted: \t{:?}", numbers);

    let mut numbers = rand_vec(10);
    println!("Before: \t\t{:?}", numbers);
    merge_sort(&mut numbers);
    println!("Merge sorted: \t\t{:?}", numbers);

    let mut numbers = rand_vec(10);
    println!("Before: \t\t{:?}", numbers);
    quick_sort(&mut numbers);
    println!("Quick sorted: \t\t{:?}", numbers);

    let mut numbers = rand_vec(10);
    println!("Before: \t\t{:?}", numbers);
    stooge_sort(&mut numbers);
    println!("Stooge sorted: \t\t{:?}", numbers);
}

fn selection_sort<T: Ord>(v: &mut [T]) {
    let (mut i, len) = (0, v.len());
    while i < len {
        let (mut j, mut cur_min) = (i + 1, i);
        while j < len {
            if v[j] < v[cur_min] {
                cur_min = j;
            }
            j = j + 1;
        }
        v.swap(i, cur_min);
        i = i + 1;
    }
}

fn insertion_sort<T: Ord>(v: &mut [T]) {
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j] < v[j-1] {
            v.swap(j, j-1);
            j = j-1;
        }
    }
}

fn merge_sort<T: Copy + Ord>(v: &mut [T]) {
    let n = v.len();
    let m = n / 2;
 
    if n <= 1 {
        return;
    }
    merge_sort(&mut v[0..m]);
    merge_sort(&mut v[m..n]);
 
    let mut y: Vec<T> = v.to_vec();
 
    merge(&v[0..m], &v[m..n], &mut y[..]);
 
    v.copy_from_slice(&y);
}
fn merge<T: Copy + PartialOrd>(v1: &[T], v2: &[T], y: &mut [T]) {
    assert_eq!(v1.len() + v2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            y[k] = v1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = v2[j];
            k += 1;
            j += 1;
        }
    }
    if i < v1.len() {
        y[k..].copy_from_slice(&v1[i..]);
    }
    if j < v2.len() {
        y[k..].copy_from_slice(&v2[j..]);
    }
}

fn quick_sort<T: Ord>(v: &mut [T]) {
 
    let len = v.len();
    if len < 2 {
        return;
    }

    let pivot_index = len / 2;
    v.swap(pivot_index, len - 1);

    let mut store_index = 0;
    for i in 0..len - 1 {
        if v[i] < v[len - 1] {
            if i != store_index { v.swap(i, store_index); }
            store_index += 1;
        }
    }
    
    v.swap(store_index, len - 1);
    let pivot_index = store_index;
 
    // Sort the left side
    quick_sort(&mut v[0..pivot_index]);
 
    // Sort the right side
    quick_sort(&mut v[pivot_index + 1..len]);
}

fn stooge_sort<T: PartialOrd>(v: &mut [T]) 
{
    let len = v.len();
 
    if v.first().unwrap() > v.last().unwrap() {
        v.swap(0, len - 1);
    }
    if len - 1 > 1 {
        let t = len / 3;
        stooge_sort(&mut v[..len - 1]);
        stooge_sort(&mut v[t..]);
        stooge_sort(&mut v[..len - 1]);
    }
}