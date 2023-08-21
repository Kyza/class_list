use crate::utils::normalize_class_list_string;

/// A trait which designates toggle types.
///
/// [`bool`], [`Option`], [`Result`], and [`Fn`] types for them are supported.
///
/// When implementing it yourself, it should return a [`bool`] which is the truthy version of the value.
///
/// - `Some(x)` -> `true`
/// - `Some(true)` -> `true`
/// - `Some(false)` -> `false`
/// - `None` -> `false`
pub trait ClassToggle {
	/// From [`ClassToggle`].
	///
	/// Converts the type into a boolean.
	///
	/// - `Some(x)` -> `true`
	/// - `Some(true)` -> `true`
	/// - `Some(false)` -> `false`
	/// - `None` -> `false`
	fn to_class_toggle(&self) -> bool;
}

impl<T, F> ClassToggle for F
where
	F: Fn() -> T,
	T: ClassToggle,
{
	fn to_class_toggle(&self) -> bool {
		self().to_class_toggle()
	}
}

impl<T, E> ClassToggle for Result<T, E>
where
	T: ClassToggle,
{
	fn to_class_toggle(&self) -> bool {
		if let Ok(value) = self {
			value.to_class_toggle()
		} else {
			false
		}
	}
}

impl<T> ClassToggle for Option<T>
where
	T: ClassToggle,
{
	fn to_class_toggle(&self) -> bool {
		if let Some(value) = self {
			value.to_class_toggle()
		} else {
			false
		}
	}
}

impl ClassToggle for bool {
	fn to_class_toggle(&self) -> bool {
		*self
	}
}

/// A trait which designates classlistable types.
///
/// [`str`], [`String`], [`Option`], [`Result`], and [`Fn`] types for them are supported.
///
/// When implementing it yourself, it should return a trimmed string of the class names separated by only one space.
///
/// - ✅ `"class"`
/// - ✅ `"class list"`
/// - ❌ `" class list "`
/// - ❌ `"class ​ ​ list"`
pub trait ClassList {
	/// From [`ClassList`].
	///
	/// Converts the type into a normalized class list string.
	///
	/// `" class ​ ​ list ​"` -> `"class list"`
	fn to_class_list(&self) -> String;
}

impl<T, F> ClassList for F
where
	F: Fn() -> T,
	T: ClassList,
{
	fn to_class_list(&self) -> String {
		self().to_class_list()
	}
}

impl<T, E> ClassList for Result<T, E>
where
	T: ClassList,
{
	fn to_class_list(&self) -> String {
		if let Ok(value) = self {
			value.to_class_list()
		} else {
			String::default()
		}
	}
}

impl<T> ClassList for Option<T>
where
	T: ClassList,
{
	fn to_class_list(&self) -> String {
		if let Some(value) = self {
			value.to_class_list()
		} else {
			String::default()
		}
	}
}

impl ClassList for String {
	fn to_class_list(&self) -> String {
		normalize_class_list_string(self.clone())
	}
}
impl ClassList for &str {
	fn to_class_list(&self) -> String {
		normalize_class_list_string(self.to_string())
	}
}
impl ClassList for str {
	fn to_class_list(&self) -> String {
		normalize_class_list_string(self.to_string())
	}
}
