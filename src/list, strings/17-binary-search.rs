// Implement binary search.

fn bin_search<T: Ord>(v: &[T], val: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = v.len();
    while low <= high {
        let mid = low + (high - low) / 2;
        if v[mid] > *val {
            high = mid - 1;
        } else if v[mid] < *val {
            low = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

fn main() {
    let vec = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    let val = 5;
    println!("bin_search found {} at index {}", &val, bin_search(&vec, &val).unwrap()); 
}