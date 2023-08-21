use class_list::{
	class_list,
	traits::{ClassList, ClassToggle},
};

#[test]
fn to_class_list() {
	assert_eq!(
		"  class    list  ".to_class_list(),
		"class list".to_string()
	);
	assert_eq!(
		"  class    list  ".to_string().to_class_list(),
		"class list".to_string()
	);
}

#[test]
fn reactive() {
	assert_eq!(
		class_list![
			"class" <=> true,
			"hidden" <=> false,
			"list" <=> true,
		](),
		"class list".to_string()
	);
}

#[test]
fn raw() {
	assert_eq!(
		class_list![
			raw;
			"class" <=> true,
			"hidden" <=> false,
			"list" <=> true,
		],
		"class list".to_string()
	);
	assert_eq!(
		class_list![
			raw = true;
			"class" <=> true,
			"hidden" <=> false,
			"list" <=> true,
		],
		"class list".to_string()
	);
	assert_eq!(
		class_list![
			raw = false;
			"class" <=> true,
			"hidden" <=> false,
			"list" <=> true,
		](),
		"class list".to_string()
	);
}

#[test]
fn clone() {
	let class = "class";
	let hidden = "hidden";
	let list = "list";
	assert_eq!(
		class_list![
			clone[class, list];
			class,
			hidden <=> false,
			list
		](),
		"class list".to_string()
	);
	assert_eq!(
		class_list![
			raw;
			clone[class, list];
			class,
			hidden <=> false,
			list
		],
		"class list".to_string()
	);
}

#[test]
fn new_impl() {
	struct Bool(bool);

	impl ClassList for Bool {
		fn to_class_list(&self) -> String {
			if self.0 {
				"true".into()
			} else {
				"false".into()
			}
		}
	}
	impl ClassToggle for Bool {
		fn to_class_toggle(&self) -> bool {
			self.0
		}
	}

	assert_eq!(
		class_list![
			move || Bool(false),
			Bool(true),
			"class",
			"hidden" <=> move || Bool(false),
			"list",
		](),
		"false true class list".to_string()
	);
}
