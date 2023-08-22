/// A function that normalizes a string's spaces.
///
/// `" class ​ ​ list ​"` -> `"class list"`
pub fn normalize_class_list_string(class_string: String) -> String {
	// Shortcuts like assuming only `' '` is whitespace and the
	// string will be the same length of shorter can be taken.
	let mut result = String::with_capacity(class_string.len());

	// Perform only one iteration over the old string.
	let mut chars = class_string.chars();
	let mut start = true;
	while let Some(char) = chars.next() {
		if start {
			// Ignore all spaces at the start of the string.
			if char != ' ' {
				start = false;
				result.push(char);
			}
		} else {
			if char == ' ' {
				// Only add the space once the next real character has been reached.
				// If that never happens then the end gets trimmed.
				// If that does happen then only one space gets added to the String.
				while let Some(next_char) = chars.next() {
					if next_char != ' ' {
						result.push(char);
						result.push(next_char);
						break;
					}
				}
			} else {
				result.push(char);
			}
		}
	}

	result
}
