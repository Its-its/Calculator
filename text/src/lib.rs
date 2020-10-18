#![warn(warnings, rust_2018_idioms, unsafe_code, dead_code)]
#![warn(clippy::all)]
#![allow(clippy::missing_docs_in_private_items, unsafe_code)]

use conversion_parser::{Factory, Parser, Tokenizer, TokenSlicer, Value, ExprToken};

pub mod error;

pub use error::{Result, Error};

#[derive(Debug)]
pub enum TextValue {
	Parsed(Value),
	Text(ExprToken)
}


// Parses text
pub fn parse(text: &str) -> Result<()> {
	println!(r#"Parsing "{}""#, text);

	let factory = Factory::default();

	let mut tokenizer = Tokenizer::new(text, &factory);
	tokenizer.parse()?;

	println!("{:?}", tokenizer.get_tokens());

	let mut parser = Parser::new_with_tokenizer(&factory, tokenizer);
	parser.options.ignore_custom_units = true;
	parser.options.ignore_exact_unit_name = true;
	parser.options.ignore_white_space = true;

	let mut slicer = TokenSlicer::new(parser.get_parsed_tokens().into_iter().filter(|e| e != &ExprToken::Whitespace).collect());

	let mut values = Vec::new();

	loop {
		slicer.forward();

		let start_pos = slicer.get_pos();

		let current_operation = parser.parse_number_expression(&mut slicer)?;

		if let Some(operation) = current_operation {
			values.push(TextValue::Parsed(operation.args.eval()?));
		} else if slicer.get_pos() == start_pos {
			let tokens = slicer.get_tokens();

			if tokens.len() == start_pos {
				values.append(&mut tokens[start_pos..].iter().map(|i| TextValue::Text(i.clone())).collect());
			} else {
				values.append(&mut tokens[start_pos..start_pos + 1].iter().map(|i| TextValue::Text(i.clone())).collect());
			}

			slicer.next_pos();
		} else {
			values.append(&mut slicer.get_tokens()[start_pos..slicer.get_pos()].iter().map(|i| TextValue::Text(i.clone())).collect());
		}

		if slicer.is_finished() {
			break;
		}
	}

	println!("-----");
	let mut underlines = String::new();

	for value in values {
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

	Ok(())
}