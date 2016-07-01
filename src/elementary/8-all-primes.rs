// Write a program that prints all prime numbers.
// (Note: if your programming language does not support arbitrary size numbers,
//  printing all primes up to the largest number you can easily represent is fine too.)

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};

struct BitVec(Vec<u8>);

impl BitVec {
    // Constructs a new BitVec struct
    pub fn new(len: usize) -> BitVec {
        BitVec(vec![0b11111111; (len / 8) + 1]) /* Initialize the vector with 1's, length / 8 to account for bytes -> bits, 
                                                +  1 to account for sizes that arent divisible by 8 (non whole integers get rounded down) */
    }
    // Set n:th bit to 0
    pub fn set(&mut self, n: usize) {
        self.0[n / 8] &= !(1u8 << (n % 8));
        // self.0[n / 8] |= (1u8 << (n % 8)); // For reserver operation (Setting to 1)
    }
    // Check if the n:th bit is set to 1 (not 0)
    pub fn is_one(&self, n: usize) -> bool {
        self.0[n / 8] & (1u8 << (n % 8)) != 0
    }
}

fn main() {
    let path = Path::new("primes.txt");
    let display = path.display();

    // Open a file in write-only mode
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let search_limit: usize;

    // Ask for search limit from user, loop until acceptable input is given
    loop {

        println!("How many integers do you want to check for primes?");

        let mut s_input = String::new();

        io::stdin().read_line(&mut s_input).expect("Failed to read line.");
            
        match s_input.trim().parse() {
            Ok(n)   => { search_limit = n; break; },
            Err(_)  => { println!("Please input a natural number."); continue; },
        };

    };

    //const SEARCH_LIMIT: usize = 100; // Can be used instead of user input
 
    // Create a new bit vector, which takes
    let mut primes = BitVec::new(search_limit);
    primes.set(0); // 0 and
    primes.set(1); // 1 aren't primes
    
    // We calculate the primes using a stride (increment) and marking off multiples 
    for stride in 2..(search_limit/2) {
        let mut n = stride; // Start from stride 
        while n < (search_limit - stride) {
            n += stride;    // Add multiples of stride
            primes.set(n); // set the bit in primes bit vector at position n to false
            // Multiples of stride cannot be prime, as they can be divided by stride
        }
    }
    // turn primes array into interation, and enumerate through it
    // if the pointer to the array at index is true, the index is the prime
    for n in 2..search_limit {
        if primes.is_one(n) { // !primes... if vector was initialized with 0's instead                
            // Write the primes to `file`
            let line = n.to_string() + "\n";
            match file.write_all(line.as_bytes()) {
                Ok(_) => println!("Wrote {} to {}", n, display),
                Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            }
        } 
    }
}
