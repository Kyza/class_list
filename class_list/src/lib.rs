#![doc = include_str!("../../README.md")]

#[doc(hidden)]
/// Not part of the public API.
pub use class_list_macro::class_list as __class_list;

#[macro_export]
#[doc = include_str!("../../README.md")]
macro_rules! class_list {
	( $($input:tt)* ) => ({
		$crate::__class_list! {
			crate = $crate;
			$($input)*
		}
	})
}

pub mod traits;

#[doc(hidden)]
/// Not part of the public API.
pub mod utils;
