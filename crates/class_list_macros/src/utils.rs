use syn::parse::{Parse, ParseBuffer, ParseStream};

pub fn peek_any<T: Parse>(
	input: ParseStream,
	parse_original: bool,
) -> Option<(ParseBuffer, T)> {
	let finput = input.fork();
	if let Ok(any) = finput.parse::<T>() {
		if parse_original {
			_ = input.parse::<T>();
		}

		Some((finput, any))
	} else {
		None
	}
}

pub fn parse_until<T: Parse>(
	input: ParseStream,
	callback: fn(input: ParseStream) -> bool,
) -> syn::Result<Vec<T>> {
	let mut tokens = vec![];

	while !callback(input) && !input.is_empty() {
		let next = match peek_any::<T>(input, true) {
			Some(token) => token.1,
			None => {
				return Err(syn::Error::new(
					input.cursor().span(),
					format!("failed to parse until {:#?}", callback),
				));
			}
		};
		tokens.push(next);
	}

	Ok(tokens)
}
