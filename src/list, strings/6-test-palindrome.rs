// Write a function that tests whether a string is a palindrome.

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
	palindrome("race car");
	palindrome("A man, a plan, a canal, Panama!");
	palindrome("Fešná paní volá: má málo vína pan šéf?");
	palindrome("notśooston");
}

fn palindrome(input: &str) {
	// Print the whole word
	println!("Input: {}", input);

	// Print the graphemes
	print!("Graphemes: ");
	for s in input.unicode_words().flat_map(|s| s.graphemes(true)) {
		print!("{} ", s);
	}
	println!("");

	// Print if palindrome
	println!("Palindrome? {}", is_palindrome(input));

	println!("");
}

fn is_palindrome(input: &str) -> bool {
	// https://en.wikipedia.org/wiki/Palindrome
	// "Allowances may be made for adjustments to capital letters, punctuation, and word dividers."

	// Yay iterators
	input.unicode_words().flat_map(|s| s.graphemes(true))
		.zip(input.unicode_words().flat_map(|s| s.graphemes(true)).rev())
		.all(|(forward, back)| {
			// FIXME! I need a comparison that ignores case, accents, 'n stuff
			forward.to_uppercase() == back.to_uppercase()
		})
}
