// Write a program that prints the next 20 leap years.

fn is_leap_year(year: i32) -> bool {
	year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
	// These iterators are really ergonomic!
	for year in (2030..).filter(|&year| is_leap_year(year)).take(20) {
		println!("{} is a leap year.", year);
	}
}
