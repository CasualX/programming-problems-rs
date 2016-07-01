// Write a function that computes the list of the first 100 Fibonacci numbers.

extern crate num;

use num::One;
use std::mem;
use std::ops::Add;
 
struct Fib<T> {
    curr: T,
    next: T,
} 
 
impl<T> Fib<T> where T: One {
    fn new() -> Self {
        Fib {curr: T::one(), next: T::one()}
    }
}
 
impl<T> Iterator for Fib<T> 
where T: Clone, for <'a> &'a T: Add<Output=T> {
    type Item = T;
 
    fn next(&mut self) -> Option<Self::Item>{
        mem::swap(&mut self.next, &mut self.curr);
        self.next = &self.next + &self.curr;
        Some(self.curr.clone())
    }
}
 
fn main() {
    // Overflows after 91
    for i in Fib::<u64>::new().take(91) {
        println!("{}", i);
    }
}