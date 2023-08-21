use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;

use crate::tokens::ClassListToken;

pub fn class_list(input: TokenStream) -> TokenStream {
	let statement = parse_macro_input!(input as ClassListToken);

	let macro_crate = statement.macro_crate;
	let imports = quote! {
		use #macro_crate::traits::*;
	};

	let clones_statements: Vec<_> = statement
		.clones
		.iter()
		.map(|name| {
			quote! {
				let #name = #name.clone();
			}
		})
		.collect();

	let values: Vec<_> = statement
		.values
		.iter()
		.map(|class| {
			let value = &class.value;
			let signal = &class.toggle;

			if let Some(signal) = signal {
				quote! {
					if (#signal).to_class_toggle() {
						(#value).to_class_list(false)
					} else {
						"".to_string()
					}
				}
			} else {
				quote! {
					(#value).to_class_list(false)
				}
			}
		})
		.collect();
	let caller = quote! {
		Vec::<String>::from([#(#values),*]).join(" ").to_class_list(true)
	};

	let code = if !statement.raw {
		quote! {
			{
				#imports
				#(#clones_statements)*
				move || #caller
			}
		}
	} else {
		quote! {
			{
				#imports
				#(#clones_statements)*
				#caller
			}
		}
	};

	code.into()
}
