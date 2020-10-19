#![warn(warnings, rust_2018_idioms, unsafe_code, dead_code)]
#![warn(clippy::all)]
#![allow(clippy::missing_docs_in_private_items, unsafe_code)]

use conversion_parser::{Factory, Parser, Tokenizer, TokenSlicer, ExprToken};

pub mod error;
pub mod cmp;
pub mod text;

pub use error::{Result, Error};
pub use text::{TextStructure, TextValue};


// Parses text
pub fn parse<'a>(text: &'a str, factory: Factory) -> Result<TextStructure<'a>> {
	println!(r#"Parsing "{}""#, text);

	let mut tokenizer = Tokenizer::new(text, &factory);
	tokenizer.parse()?;

	let mut parser = Parser::new_with_tokenizer(&factory, tokenizer);
	parser.options.ignore_custom_units = true;
	parser.options.ignore_exact_unit_name = true;
	parser.options.ignore_white_space = true;

	let mut slicer = TokenSlicer::new(parser.get_parsed_tokens().into_iter().filter(|e| e != &ExprToken::Whitespace).collect());

	let mut parsed = Vec::new();

	loop {
		slicer.forward();

		let start_pos = slicer.get_pos();

		let current_operation = parser.parse_number_expression(&mut slicer)?;

		if let Some(operation) = current_operation {
			parsed.push(TextValue::Parsed(operation.args.eval()?));
		} else if slicer.get_pos() == start_pos {
			let tokens = slicer.get_tokens();

			if tokens.len() == start_pos {
				parsed.append(&mut tokens[start_pos..].iter().map(|i| TextValue::Text(i.clone())).collect());
			} else {
				parsed.append(&mut tokens[start_pos..start_pos + 1].iter().map(|i| TextValue::Text(i.clone())).collect());
			}

			slicer.next_pos();
		} else {
			parsed.append(&mut slicer.get_tokens()[start_pos..slicer.get_pos()].iter().map(|i| TextValue::Text(i.clone())).collect());
		}

		if slicer.is_finished() {
			break;
		}
	}

	println!("-----");
	let mut underlines = String::new();

	for value in &parsed {
		match value {
			TextValue::Parsed(v) => {
				let t = v.to_string();

				print!("{}", t);

				underlines.push_str(&(0..t.len()).map(|_| "-").collect::<String>());
			}

			TextValue::Text(e) => {
				let t = e.to_string();

				print!("{}", t);

				underlines.push_str(&(0..t.len()).map(|_| "_").collect::<String>());
			}
		}

		print!(" ");
		underlines.push_str("_");
	}

	println!("\n{}", underlines);

	Ok(TextStructure::new(
		text,
		parsed,
		parser.tokenizer.get_compiled().to_vec()
	))
}