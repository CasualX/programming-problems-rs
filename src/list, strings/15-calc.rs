// Write functions that add, subtract, and multiply two numbers in their digit-list representation (and return a new digit list). 
// If you’re ambitious you can implement Karatsuba multiplication. Try different bases. What is the best base if you care about speed? 
// If you couldn’t completely solve the prime number exercise above due to the lack of large numbers in your language, you can now use your own library for this task.

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::cmp::max;
use std::mem;
#[allow(unused_imports)]
use std::io;
use std::io::Write;
use std::fmt;

#[derive(Debug, Clone)]
struct DigitCalc {
	digits: Vec<i8>,
	sign: Sign,
}

#[derive(Debug, Clone, PartialEq)]
enum Sign {
	Negative,
	Positive,
}

impl DigitCalc {
	pub fn new(x: i64) -> DigitCalc {
		let mut vec: Vec<i8> = Vec::new();
		let mut sign: Sign = Sign::Positive;

		for i in x.to_string().chars() { // Convert digits to individual chars of a string
			match i {
				'-' => sign = Sign::Negative,
				_ 	=> vec.push(i.to_digit(10).unwrap() as i8), // Push each char to vec as u8 digit (in base 10)
			}	
		}

		DigitCalc {
			digits: vec,
			sign: sign,
		}
	}

	fn set_sign(mut self, sign: Sign) -> DigitCalc {
		self.sign = sign;
		self
	}

	fn is_larger(&self, other: &DigitCalc) -> bool {
		let mut is_larger = false;

		if self.digits.len() > other.digits.len() { // Larger length == larger number
			is_larger = true;
		} else if self.digits.len() == other.digits.len() { // replace with iterators?
			for i in 0..self.digits.len() {
				if self.digits[i] > other.digits[i] { // if most significant digit is larger, the number as a whole is larger
					is_larger = true;
					break;
				} else if self.digits[i] == other.digits[i] { // If the digits are the same, check the next digit
					continue;
				} else {
					break;
				}
			}
		}
		is_larger
	}

	pub fn is_positive(&self) -> bool {
		match self.sign {
			Sign::Positive => true,
			Sign::Negative => false,
		}
	}
}

impl fmt::Display for DigitCalc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
        	Sign::Positive => write!(f, "+{:?}", self.digits),
        	Sign::Negative => write!(f, "-{:?}", self.digits),
        } 
    }
}

impl Add for DigitCalc {
	type Output = DigitCalc;

	fn add(mut self, mut other: DigitCalc) -> DigitCalc {
		let mut sign = Sign::Positive;
		// Sort the inputted numbers based on their signs
		if self.is_positive() {
			if other.is_positive() { // Both are positive
				// a + b == Do nothing
			} else { // Self is positive, other is negative 
				other = other.set_sign(Sign::Positive); // All negative signs need to be changed to positive
				let averted_res = self.clone() - other.clone(); // a - b
				return averted_res
			} 
		} else { 
			if other.is_positive() { // Self is negative, other is positive
				self = self.set_sign(Sign::Positive);
				let averted_res = other.clone() - self.clone(); // b - a
				return averted_res
			} else { // Both are negative 
				self = self.set_sign(Sign::Positive);
				other = other.set_sign(Sign::Positive);
				sign = Sign::Negative; // -(a + b)
			} 
		}

		self.digits.reverse(); // Reverse in place
 		other.digits.reverse(); // Reversed vectors are easier to work with, since we want to start with the least significant digits
		let mut result_vec: Vec<i8> = Vec::with_capacity(self.digits.len() + other.digits.len()); // Preallocate for max case scenario
		let length = max(self.digits.len(), other.digits.len()); // For loop needs run for the length of the longer vector
		let mut carry = 0;

		for i in 0..length + 1 { // + 1 loop for carry
			// Start with 0's
			let mut a = 0; 
			let mut b = 0;
			// Get the digits, if they're found
			if let Some(&x) = self.digits.get(i) {
				a = x;
			}

			if let Some(&x) = other.digits.get(i) {
				b = x;
			}

			let sum = a + b + carry;

			carry = sum / 10; // Divide by 10 to get the first digit only (Rounds to 1 or 0)

			result_vec.push(sum % 10); // Mod 10 to get last digit only
		}
		// Trim leading 0's
		if result_vec[result_vec.len() - 1] == 0 {
			result_vec.pop();
		} 
		// Reverse back to original format
		result_vec.reverse();

		DigitCalc { 
			digits: result_vec, 
			sign: sign,

		}
	}
}

impl Sub for DigitCalc {
	type Output = DigitCalc;

	fn sub(mut self, mut other: DigitCalc) -> DigitCalc {
		// Sort the inputted numbers based on their signs
		if self.is_positive() {
			if other.is_positive() { // Both are positive
				// a - b == Do Nothing
			} else { // Self is positive, other is negative 
				other = other.set_sign(Sign::Positive);
				let averted_res = self.clone() + other.clone(); // a + b
				return averted_res
			} 
		} else { 
			if other.is_positive() { // Self is negative, other is positive
				self = self.set_sign(Sign::Positive);
				let averted_res = self.clone() + other.clone();
				let averted_res = DigitCalc {
					digits: averted_res.digits, // -(a + b)
					sign: Sign::Negative,
				};
				return averted_res
			} else { // Both are negative 
				self = self.set_sign(Sign::Positive);
				other = other.set_sign(Sign::Positive);
				mem::swap(&mut self.digits, &mut other.digits); // b - a
			} 
		}

		let mut sign = Sign::Positive;

		if !&self.is_larger(&other) {
			mem::swap(&mut self.digits, &mut other.digits);
			sign = Sign::Negative;
		}

		self.digits.reverse(); // Reverse in place
 		other.digits.reverse(); // Reversed vectors are easier to work with, since we want to start with the least significant digits
		let mut result_vec: Vec<i8> = Vec::with_capacity(self.digits.len() + other.digits.len()); // Preallocate for max case scenario
		let length = max(self.digits.len(), other.digits.len()); // For loop needs run for the length of the longer vector
		let mut carry = 0;
		
		for i in 0..length + 1 { // + 1 loop for carry
			// Start with 0's
			let mut a = 0; 
			let mut b = 0;
			// Get the digits, if they're found
			if let Some(&x) = self.digits.get(i) {
				a = x;
			}

			if let Some(&x) = other.digits.get(i) {
				b = x;
			}

			let mut sum: i8;

			sum = a - b - carry;

			if sum < 0 {
				carry = 1;
				sum = 10 + sum;
			} else {
				carry = 0;
			}

			result_vec.push(sum);
		}
		// Trim leading 0's
		if result_vec[result_vec.len() - 1] == 0 {
			result_vec.pop();
		} 
		// Reverse back to original format
		result_vec.reverse();

		DigitCalc { 
			digits: result_vec, 
			sign: sign,

		}
	}
}

impl Mul for DigitCalc {
	type Output = DigitCalc;

	fn mul(mut self, mut other: DigitCalc) -> DigitCalc {
		self.digits.reverse(); // Reverse in place
		other.digits.reverse(); // Reversed vectors are easier to work with, since we want to start with the least significant digits

 		let mut result_vec: Vec<i8> = vec![0; self.digits.len() + other.digits.len()]; // Unlike in addition, we need to initialize the vector with 0's since we can't just push a new digit every time.
 		let mut carry = 0;
 		let mut sign = Sign::Positive;

 		// Figure out the sign of the final product first
 		if self.sign == Sign::Negative || other.sign == Sign::Negative {
 			sign = Sign::Negative;
 		} else if self.sign == Sign::Negative && other.sign == Sign::Negative {
 			sign = Sign::Positive;
 		}

		for i in 0..self.digits.len() { // Nested loops for grid method of multiplication
			let mut a = 0;
			if let Some(&x) = self.digits.get(i) { 
					a = x;
			}
			for j in 0..other.digits.len() + 1 { // + 1 loop for carry
				let mut b = 0;
				if let Some(&x) = other.digits.get(j) {
					b = x;
				}

				let product = result_vec[i + j] + (a * b) + carry; // Take previous result, add current product and the carry. 

				carry = product / 10; // Divide by 10 to get the first digit only

				result_vec[i + j] = product % 10; // Mod 10 to get the last digit only
			}
		}
		// Trim leading 0's 
		if result_vec[result_vec.len() - 1] == 0 {
			result_vec.pop();
		} 

		result_vec.reverse();

		DigitCalc { 
			digits: result_vec, 
			sign: sign,
		}
	}
}

fn main() {
	let a = DigitCalc::new(123);
	let b = DigitCalc::new(-15);
	let c = a.clone() + b.clone();
	let d = a.clone() - b.clone();
	let e = a.clone() * b.clone();

	println!("({}) + ({}) = {}", a, b, c);
	println!("({}) - ({}) = {}", a, b, d);
	println!("({}) * ({}) = {}", a, b, e);

	// let user_num1;
	// let user_num2;
	// let user_oper; 

	// loop {
	// 	print!("Input first number (Signed integer): ");
	// 	io::stdout().flush().unwrap();

	// 	let mut num1 = String::new();
	// 	io::stdin().read_line(&mut num1).expect("Failed to read line");

	// 	user_num1 = match num1.trim().parse() {
	// 		Ok(n) 	=> n,
	// 		Err(_) 	=> { println!("Not a valid integer."); continue; },
	// 	};
	// 	break;
	// }

	// loop {
	// 	print!("Input second number (Signed integer): ");
	// 	io::stdout().flush().unwrap();

	// 	let mut num2 = String::new();
	// 	io::stdin().read_line(&mut num2).expect("Failed to read line");

	// 	user_num2 = match num2.trim().parse() {
	// 		Ok(n) 	=> n,
	// 		Err(_) 	=> { println!("Not a valid integer."); continue; },
	// 	};
	// 	break;
	// }
	// loop {
	// 	print!("Input operator (+, - or *): ");
	// 	io::stdout().flush().unwrap();

	// 	let mut oper = String::new();
	// 	io::stdin().read_line(&mut oper).expect("Failed to read line");

	// 	user_oper = match oper.trim().as_ref() {
	// 		"+" 	=> "+",
	// 		"-" 	=> "-",
	// 		"*" 	=> "*",
	// 		_ 	=> { println!("Not a valid operator."); continue; },
	// 	};
	// 	break;
	// }

	// let a = DigitCalc::new(user_num1);
	// let b = DigitCalc::new(user_num2);

	// let c = match user_oper {
	// 	"+" => a.clone() + b.clone(),
	// 	"-" => a.clone() - b.clone(),
	// 	"*" => a.clone() * b.clone(),
	// 	_ => panic!("shit"),
	// };


	// println!("{} {} {} = ", a, user_oper, b);
	// println!("{} ", c);

}
