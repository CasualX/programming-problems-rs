// Write function that translates a text to Pig Latin and back. 
// English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’.
// “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

fn to_piglatin(trs_str: &str) -> String {
	let mut string = String::new();
	let mut first_char = None;
	let mut punctuation;
	let mut titlecased;

	for word in trs_str.split_whitespace() {
		punctuation = None;
		titlecased = false;
		for (i, chr) in word.chars().enumerate() {
			if i == 0 {
				first_char = Some(chr);
				if chr.to_string() == chr.to_string().to_uppercase() {
					titlecased = true;
				}

			} else if i == 1 && titlecased {
				string.push_str(&chr.to_string().to_uppercase())
			} else if i == word.len() - 1 && !chr.is_alphabetic() {
				punctuation = Some(chr);
			} else {
				string.push_str(&chr.to_string());
			}
		}

		if let Some(x) = first_char {
			string.push_str(&x.to_string().to_lowercase());
		}

		string.push_str("ay");

		if let Some(x) = punctuation {
			string.push_str(&x.to_string());
		}

		string.push_str(" ");
	}
	string
}

// TODO: Translate back from pig Latin to English, handle uppercase and non alphabetic characters better.

fn main() {
	let test_string = "I'm calling you a cunt in pig latin.";
	println!("{}", to_piglatin(test_string));
}