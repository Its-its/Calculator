use console::Term;

use conversion_parser::{Factory, ExprToken, Operator};

pub mod error;
pub mod display;
pub mod commands;

pub use error::{Result, Error};
pub use display::{ColorTypes, colorize_tokens, space_tokens};
pub use commands::{Command, get_command};


fn main() {
	let term = Term::stdout();

	let mut factory = Factory::new();

	while let Ok(line) = term.read_line() {
		parse_line(&line, &mut factory, &term)
		.expect("parse_line");
	}
}


fn parse_line(line: &str, factory: &mut Factory, term: &Term) -> Result<()> {
	let value = factory.parse(line)?;

	let tokens = value.into_tokens();

	// Check to see if it's possibly a command.
	if tokens.len() == 1 {
		if let Some(ExprToken::Literal(value)) = tokens.get(0) {
			if let Some(cmd) = get_command(value) {
				let _ = term.write_line(&cmd.display(factory));
				return Ok(());
			}
		}
	}

	// Check if trying to set constant.
	else if tokens.len() == 3 {
		let mut iter = tokens.iter().peekable();

		let name = iter.next().unwrap();
		let op = iter.next().unwrap();
		let value = iter.next().unwrap();

		// TODO: Expand to be able to use something like "x = PI * 180"
		if name.is_literal() && op.is_operator() && op.as_operator() == &Operator::Equal && value.is_number() {
			factory.add_constant(name.clone().into_literal(), value.clone().into_number());
		}
	}

	let tokens = space_tokens(tokens);

	let _ = term.write_line(&colorize_tokens(tokens));

	Ok(())
}