#![doc = include_str!("../README.md")]
#![allow(clippy::tabs_in_doc_comments)]

use proc_macro::TokenStream;

mod class_list;
mod tokens;
mod utils;

#[proc_macro]
#[doc = include_str!("../README.md")]
pub fn class_list(input: TokenStream) -> TokenStream {
	class_list::class_list(input)
}
