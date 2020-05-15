use std::fmt;
use std::str::Chars;

use web_sys::*;

use conversion_parser::{ExprToken, Tokenizer};

use crate::create_element;


pub trait LineDisplay {
	fn render(&self) -> HtmlDivElement;
}


// Line

pub struct Line<'a> {
	total_width: f64,

	value: &'a [ExprToken]
}

impl<'a> Line<'a> {
	pub fn new<'b: 'a>(value: &'b [ExprToken]) -> Self {
		Self {
			total_width: 0.0,
			value
		}
	}
}

impl<'a> fmt::Display for Line<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Line Goes Here...")
	}
}

impl<'a> LineDisplay for Line<'a> {
	fn render(&self) -> HtmlDivElement {
		let container: HtmlDivElement = create_element("div");
		container.class_list().add_1("line");

		for token in self.value {
			let value: HtmlSpanElement = create_element("span");

			if token.is_operator() {
				value.set_inner_text(&format!(" {} ", token));
			} else if let ExprToken::Number(n) = token {
				value.set_inner_text(&format!("{}", n));
			} else if token.is_literal() {
				value.set_inner_text(&format!(" {}", token));
			} else {
				value.set_inner_text(&format!("{}", token));
			}

			match token {
				ExprToken::Literal(_) => {value.class_list().add_1("literal");},
				ExprToken::Number(_) => {value.class_list().add_1("amount");},
				ExprToken::Operator(_) => {value.class_list().add_1("operator");},
				_ => {}
			}

			container.append_child(&value);
		}

		container
	}
}


// Table

pub struct Table<'a> {
	total_width: Option<f64>,

	// How many cells along the X plane?
	horizontal_cell_count: usize,

	value: &'a [&'a [ExprToken]]
}

impl<'a> Table<'a> {
	pub fn new<'b: 'a>(value: &'b [&'b [ExprToken]]) -> Self {
		Self {
			total_width: None,
			horizontal_cell_count: 2,
			value
		}
	}
}

impl<'a> LineDisplay for Table<'a> {
	fn render(&self) -> HtmlDivElement {
		let container: HtmlDivElement = create_element("div");
		container.class_list().add_1("table");

		let table: HtmlElement = create_element("table");
		container.append_child(&table);

		// Starts at 1 since (0 % amount == 0) is true
		let mut index = 1;
		let mut current_row: HtmlElement = create_element("tr");
		table.append_child(&current_row);

		for line in self.value {
			let value: HtmlElement = create_element("td");

			let line = Line::new(*line);
			value.append_child(&line.render());

			current_row.append_child(&value);

			if index % self.horizontal_cell_count == 0 {
				current_row = create_element("tr");
				table.append_child(&current_row);
			}


			index += 1;
		}

		container
	}
}






pub fn pretty_print<'a>(chars: Chars<'a>) -> String {
	let decimal_place = chars.as_str().find('.').unwrap_or(chars.as_str().len());

	let rev_chars = chars.rev().enumerate();
	let mut s = String::new();

	let mut index = 0;

	for (idx, val) in rev_chars {
		if idx <= decimal_place {
			s.insert(0, val);
			continue;
		}

		// if val == '.' {
		// 	index = 0;
		// 	s.insert(0, val);
		// 	continue;
		// }

		if index != 0 && index % 3 == 0 {
			s.insert(0, ',');
		}

		s.insert(0, val);

		index += 1;
	}

	s
}