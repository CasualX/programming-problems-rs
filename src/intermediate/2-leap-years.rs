// Write a program that takes the duration of a year (in fractional days) for an imaginary planet as an input
// and produces a leap-year rule that minimizes the difference to the planet’s solar year.

use std::io::{self, Write};

const MAX_YEARS: i64 = 1000;
const EPS_YEARS: f64 = 1.0 / (MAX_YEARS as f64);

fn main() {
	print!("Year duration: ");
	io::stdout().flush().unwrap();

	let mut duration = String::new();
	io::stdin().read_line(&mut duration).unwrap();

	let duration: f64 = duration.trim().parse().unwrap();

	let mut leaps: Vec<i64> = Vec::new();
	add_leaps(&mut leaps, duration.fract());
	print_leaps(&leaps);
	check_leaps(&leaps, duration);

	let earth_duration = 365.2422;
	println!("\nFor reference, Earth leaps:\nYear duration: {}", earth_duration);
	let earth_leaps = vec![4, 100 / 4, 400 / 100];
	print_leaps(&earth_leaps);
	check_leaps(&earth_leaps, earth_duration);
}

fn print_leaps(leaps: &Vec<i64>) {
	let mut years = 1;
	let mut add = true;
	for i in leaps {
		years *= *i;
		println!("{} {} years.", if add {"Every"} else {"Except every"}, years);
		add = !add;
	}
}
fn check_leaps(leaps: &Vec<i64>, duration: f64) {
	let days = duration.floor() as i64;
	let mut time = 0;

	// Calculate after a thousand years
	for year in 0..1000 {
		time += days;
		// Check leap year rule
		let mut leap = false;
		let mut years = 1;
		for it in leaps {
			years *= *it;
			if year % years == 0 {
				leap = !leap;
			}
		}
		// Add a leap day as needed
		if leap {
			time += 1;
		}
	}

	println!("After a thousand years, difference is {} days.", duration * 1000.0 - (time as f64));
}
fn add_leaps(leaps: &mut Vec<i64>, fract: f64) {
	if fract.abs() > EPS_YEARS {
		let n = (1.0 / fract).floor();
		let rem = fract * n;
		if 1.0 - rem > EPS_YEARS {
			leaps.push(n as i64);
			sub_leaps(leaps, rem);
		}
	}
}
fn sub_leaps(leaps: &mut Vec<i64>, fract: f64) {
	let fract = 1.0 - fract;
	let n = (1.0 / fract).floor() as i64;
	if n < MAX_YEARS {
		leaps.push(n);

		let f = 1.0 - fract * (n as f64);
		if 1.0 - f > EPS_YEARS {
			add_leaps(leaps, f);		
		}
	}
}
