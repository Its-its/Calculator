use std::fmt;

use crate::operations::{ExpressionArg, Divide, Multiply, Add, Subtract, Exponentiate, Conversion, Comparison};


#[derive(Debug, PartialEq)]
pub enum TokenType {
	Comma,
	Whitespace,
	StartGrouping,
	EndGrouping,

	Number,
	Operator,
	Literal,

	ExactOperator(Operator)
}

impl PartialEq<ExprToken> for TokenType {
	fn eq(&self, other: &ExprToken) -> bool {
		match (self, other) {
			(Self::Comma, ExprToken::Comma) |
			(Self::Whitespace, ExprToken::Whitespace) |
			(Self::StartGrouping, ExprToken::StartGrouping) |
			(Self::EndGrouping, ExprToken::EndGrouping) |

			(Self::Number, ExprToken::Number(_)) |
			(Self::Operator, ExprToken::Operator(_)) |
			(Self::Literal, ExprToken::Literal(_)) => true,

			(Self::ExactOperator(o1), ExprToken::Operator(o2)) => o1 == o2,

			_ => false
		}
	}
}

impl Into<TokenType> for Operator {
	fn into(self) -> TokenType {
		TokenType::ExactOperator(self)
	}
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
	Plus,
	Minus,
	Divide,
	Division,
	Multiply,
	Caret,

	ConvertInto,

	Equal,

	GreaterThan,
	LessThan,
	GreaterThanOrEqual,
	LessThanOrEqual,
	DoubleEqual,
	DoesNotEqual,
	ApproxEqual
}

impl Operator {
	// Used for + - / * %
	pub fn compare(&self, left: ExpressionArg, right: ExpressionArg) -> ExpressionArg {
		match self {
			Operator::Plus => {
				Box::new(
					Add::new(
						left,
						right
					)
				)
			}

			Operator::Minus => {
				Box::new(
					Subtract::new(
						left,
						right
					)
				)
			}

			Operator::Multiply => {
				Box::new(
					Multiply::new(
						left,
						right
					)
				)
			}

			Operator::Divide => {
				Box::new(
					Divide::new(
						left,
						right
					)
				)
			}

			Operator::Caret => {
				Box::new(
					Exponentiate::new(
						left,
						right
					)
				)
			}

			Operator::ConvertInto => {
				Box::new(
					Conversion::new(
						left,
						right
					)
				)
			}

			op @ Operator::GreaterThan |
			op @ Operator::GreaterThanOrEqual |
			op @ Operator::LessThan |
			op @ Operator::LessThanOrEqual |
			op @ Operator::DoubleEqual |
			op @ Operator::DoesNotEqual => {
				Box::new(
					Comparison::new(
						left,
						right,
						op.clone()
					)
				)
			}

			_ => panic!("Cannot compare with this Operator")
		}
	}
}

impl fmt::Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Operator::Plus => f.write_str("+"),
			Operator::Minus => f.write_str("-"),
			Operator::Divide => f.write_str("/"),
			Operator::Division => f.write_str("%"),
			Operator::Multiply => f.write_str("*"),
			Operator::Caret => f.write_str("^"),

			Operator::ConvertInto => f.write_str("->"),

			Operator::Equal => f.write_str("="),
			Operator::GreaterThan => f.write_str(">"),
			Operator::LessThan => f.write_str("<"),
			Operator::GreaterThanOrEqual => f.write_str(">="),
			Operator::LessThanOrEqual => f.write_str("<="),
			Operator::DoesNotEqual => f.write_str("!="),
			Operator::ApproxEqual => f.write_str("~="),
			Operator::DoubleEqual => f.write_str("==")
		}
	}
}


#[derive(Debug, Clone, PartialEq)]
pub enum ExprToken {
	Comma,
	Whitespace,
	StartGrouping,
	EndGrouping,

	Number(f64),
	Operator(Operator),
	Literal(String)
}

impl ExprToken {
	pub fn is_number(&self) -> bool {
		match self {
			ExprToken::Number(_) => true,
			_ => false
		}
	}

	pub fn is_operator(&self) -> bool {
		match self {
			ExprToken::Operator(_) => true,
			_ => false
		}
	}

	pub fn is_literal(&self) -> bool {
		match self {
			ExprToken::Literal(_) => true,
			_ => false
		}
	}

	pub fn is_expr_operator(&self) -> bool {
		match self {
			ExprToken::Operator(o) => match o {
				Operator::Plus |
				Operator::Minus |
				Operator::Multiply |
				Operator::Divide |
				Operator::Division => true,
				_ => false
			},
			_ => false
		}
	}

	pub fn is_expr_caret(&self) -> bool {
		match self {
			ExprToken::Operator(Operator::Caret) => true,
			_ => false
		}
	}

	pub fn as_operator(&self) -> &Operator {
		match self {
			ExprToken::Operator(o) => o,
			_ => panic!("Not an Operator")
		}
	}

	pub fn into_operator(self) -> Operator {
		match self {
			ExprToken::Operator(o) => o,
			_ => panic!("Not an Operator")
		}
	}

	pub fn from_literal(self) -> String {
		match self {
			ExprToken::Literal(l) => l,
			_ => panic!("Not an Literal")
		}
	}
}

impl fmt::Display for ExprToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ExprToken::Comma => f.write_str(","),
			ExprToken::Whitespace => f.write_str(" "),
			ExprToken::StartGrouping => f.write_str("("),
			ExprToken::EndGrouping => f.write_str(")"),
			ExprToken::Number(v) => v.fmt(f),
			ExprToken::Operator(o) => o.fmt(f),
			ExprToken::Literal(l) => f.write_str(l)
		}
	}
}

impl Into<ExprToken> for Operator {
	fn into(self) -> ExprToken {
		ExprToken::Operator(self)
	}
}