/// A function that normalizes a string's spaces.
///
/// `" class ​ ​ list ​"` -> `"class list"`
pub fn normalize_class_list_string(class_string: String) -> String {
	// Shortcut most processing if possible.
	if !class_string.contains("  ") {
		return class_string.trim().to_string();
	}

	class_string
		.trim()
		.split(' ')
		.filter_map(|class| {
			let class = class.trim().to_string();
			if !class.is_empty() {
				Some(class)
			} else {
				None
			}
		})
		.collect::<Vec<String>>()
		.join(" ")
}
