use proc_macro2::{Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};
use syn::{
	bracketed,
	parse::{Parse, ParseBuffer, ParseStream, Parser},
	punctuated::Punctuated,
	token::Crate,
	LitBool, Path, Token,
};

use crate::utils::{parse_until, peek_any};

mod keyword {
	syn::custom_keyword!(raw);
	syn::custom_keyword!(clone);
}

#[derive(Debug, Clone)]
pub struct BindPunct;

impl Parse for BindPunct {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Token![<]>()?;
		input.parse::<Token![=]>()?;
		input.parse::<Token![>]>()?;
		Ok(BindPunct {})
	}
}

#[derive(Debug, Clone)]
pub struct CrateOverride(pub Path);

impl Default for CrateOverride {
	fn default() -> Self {
		Self(
			Parser::parse2(
				|input: &ParseBuffer<'_>| input.parse::<Path>(),
				quote! { ::class_list },
			)
			.unwrap(),
		)
	}
}

impl Parse for CrateOverride {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Crate>()?;
		input.parse::<Token![=]>()?;
		Ok(CrateOverride(input.parse::<Path>()?))
	}
}

#[derive(Debug, Clone, Default)]
pub struct RawOption(pub bool);

impl Parse for RawOption {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input.parse::<keyword::raw>()?;

		if input.peek(Token![=]) {
			input.parse::<Token![=]>()?;
			Ok(RawOption(input.parse::<LitBool>()?.value))
		} else {
			Ok(RawOption(true))
		}
	}
}

#[derive(Debug, Clone, Default)]
pub struct CloneListOption(pub Vec<Path>);

impl Parse for CloneListOption {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input.parse::<keyword::clone>()?;

		if input.peek(Token![=]) {
			input.parse::<Token![=]>()?;
		}

		let content;
		bracketed!(content in input);

		let clones =
			Punctuated::<Path, Token![,]>::parse_terminated(&content)?
				.into_iter()
				.collect::<Vec<Path>>();

		Ok(CloneListOption(clones))
	}
}

#[derive(Debug, Clone)]
pub struct ClassNameToken {
	pub value: TokenStream,
	pub toggle: Option<TokenStream>,
}

impl Parse for ClassNameToken {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let value = parse_until::<TokenTree>(input, |input| {
			input.peek(Token![,])
				|| peek_any::<BindPunct>(input, false).is_some()
		})?
		.iter()
		.map(|tt| tt.into_token_stream())
		.collect();

		let toggle = if peek_any::<Token![:]>(input, true).is_some()
			|| peek_any::<BindPunct>(input, true).is_some()
		{
			Some(
				parse_until::<TokenTree>(input, |input| {
					input.peek(Token![,])
				})?
				.iter()
				.map(|tt| tt.into_token_stream())
				.collect(),
			)
		} else {
			None
		};

		Ok(ClassNameToken { value, toggle })
	}
}

#[derive(Debug, Clone)]
pub struct ClassListToken {
	pub macro_crate: Path,
	pub raw: bool,
	pub values: Vec<ClassNameToken>,
	pub clones: Vec<Path>,
}
#[derive(Debug, Clone)]
pub enum ClassListOption {
	MacroCrate(Path),
	Raw(RawOption),
	Clones(CloneListOption),
}

impl Parse for ClassListOption {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if let Some(macro_crate) = peek_any::<CrateOverride>(input, true) {
			Ok(ClassListOption::MacroCrate(macro_crate.1 .0))
		} else if let Some((_, raw)) = peek_any::<RawOption>(input, true) {
			Ok(ClassListOption::Raw(raw))
		} else if let Some((_, clones)) =
			peek_any::<CloneListOption>(input, true)
		{
			Ok(ClassListOption::Clones(clones))
		} else {
			Err(syn::Error::new(
				Span::call_site(),
				"invalid ClassListOption",
			))
		}
	}
}

impl Parse for ClassListToken {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut macro_crate = None;
		let mut raw = false;
		let mut clones = None;

		while !input.is_empty()
			&& peek_any::<ClassListOption>(input, false).is_some()
		{
			match input.parse::<ClassListOption>()? {
				ClassListOption::MacroCrate(value) => {
					macro_crate = Some(value);
				}
				ClassListOption::Raw(value) => {
					raw = value.0;
				}
				ClassListOption::Clones(value) => {
					clones = Some(value);
				}
			}

			if input.peek(Token![;]) {
				input.parse::<Token![;]>()?;
			}
		}

		let mut values = vec![];

		while !input.is_empty() {
			values.push(input.parse::<ClassNameToken>()?);

			peek_any::<Token![,]>(input, true);
		}

		Ok(ClassListToken {
			macro_crate: macro_crate.unwrap_or(CrateOverride::default().0),
			raw,
			clones: clones.unwrap_or(CloneListOption::default()).0,
			values,
		})
	}
}
