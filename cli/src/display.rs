use conversion_parser::ExprToken;
use console::Style;

pub enum ColorTypes {
	Default,
	Types, // TODO: Better name.

	Number,
	Literal,
	Operator,
	Grouping,

	Error,
	Success
}

impl ColorTypes {
	pub fn style(&self) -> Style {
		use ColorTypes::*;

		match self {
			Default => Style::new().white(),
			Types => Style::new().blue(),

			Number => Style::new().cyan(),
			Literal => Style::new().yellow(),
			Operator => Style::new().green(),
			Grouping => Style::new().bold(),

			Error=> Style::new().red(),
			Success => Style::new().green(),
		}
	}

	pub fn str(self, val: &str) -> String {
		self
		.style()
		.apply_to(val)
		.to_string()
	}
}

impl From<&ExprToken> for ColorTypes {
	fn from(token: &ExprToken) -> Self {
		match token {
			ExprToken::Literal(_) => ColorTypes::Literal,
			ExprToken::Number(_) => ColorTypes::Number,
			ExprToken::Operator(_) => ColorTypes::Operator,
			ExprToken::StartGrouping |
			ExprToken::EndGrouping => ColorTypes::Grouping,
			_ => ColorTypes::Default
		}
	}
}

pub fn space_tokens(tokens: Vec<ExprToken>) -> Vec<ExprToken> {
	let mut spaced = Vec::new();

	let mut iter = tokens.into_iter().peekable();

	while let Some(token) = iter.next() {
		spaced.push(token.clone());

		if iter.peek().is_none() {
			break;
		}

		match token {
			ExprToken::Number(_) => if iter.peek().unwrap().is_literal() {
				continue;
			}

			ExprToken::Literal(_) => if iter.peek().unwrap().is_literal() {
				continue;
			}

			ExprToken::Whitespace => continue,

			_ => {}
		}

		spaced.push(ExprToken::Whitespace);
	}

	spaced
}

pub fn colorize_tokens(tokens: Vec<ExprToken>) -> String {
	tokens.into_iter()
	.map(|t| {
		ColorTypes::from(&t)
		.style()
		.apply_to(t)
		.to_string()
	})
	.collect::<String>()
}