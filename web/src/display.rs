use std::fmt;
use std::str::Chars;

use web_sys::*;

use conversion_parser::ExprToken;

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
		let _ = container.class_list().add_1("line");

		for token in self.value {
			let value: HtmlSpanElement = create_element("span");

			value.set_inner_text(&format!("{}", token));

			let _ = match token {
				ExprToken::Literal(_) => value.class_list().add_1("literal"),
				ExprToken::Number(_) => value.class_list().add_1("amount"),
				ExprToken::Operator(_) => value.class_list().add_1("operator"),
				ExprToken::StartGrouping | ExprToken::EndGrouping => value.class_list().add_1("grouping"),
				_ => Ok(())
			};

			let _ = container.append_child(&value);
		}

		container
	}
}


// Table

pub struct Table {
	total_width: Option<f64>,

	// How many cells along the X plane?
	horizontal_cell_count: usize,

	value: Vec<Vec<ExprToken>>
}

impl Table {
	pub fn new(value: Vec<Vec<ExprToken>>) -> Self {
		Self {
			total_width: None,
			horizontal_cell_count: 2,
			value
		}
	}
}

impl LineDisplay for Table {
	fn render(&self) -> HtmlDivElement {
		let container: HtmlDivElement = create_element("div");
		let _ = container.class_list().add_1("table");

		let table: HtmlElement = create_element("table");
		let _ = container.append_child(&table);

		// Starts at 1 since (0 % amount == 0) is true
		let mut index = 1;
		let mut current_row: HtmlElement = create_element("tr");
		let _ = table.append_child(&current_row);

		for line in &self.value {
			let value: HtmlElement = create_element("td");

			let line = Line::new(&line);
			let _ = value.append_child(&line.render());

			let _ = current_row.append_child(&value);

			if index % self.horizontal_cell_count == 0 {
				current_row = create_element("tr");
				let _ = table.append_child(&current_row);
			}


			index += 1;
		}

		container
	}
}






pub fn pretty_print(chars: Chars<'_>) -> String {
	let decimal_place = chars.as_str().find('.').unwrap_or_else(|| chars.as_str().len());

	let rev_chars = chars.rev().enumerate();
	let mut s = String::new();

	let mut index = 0;

	for (idx, val) in rev_chars {
		if idx <= decimal_place {
			s.insert(0, val);
			continue;
		}

		if index != 0 && index % 3 == 0 {
			s.insert(0, ',');
		}

		s.insert(0, val);

		index += 1;
	}

	s
}