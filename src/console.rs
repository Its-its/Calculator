use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

use conversion_parser::{Parser, Operator, ExprToken, Tokenizer};

use crate::{Line, LineDisplay, Table};


pub fn window() -> Window {
	web_sys::window().unwrap()
}

pub fn document() -> Document {
	window().document().unwrap()
}

pub fn create_element<E: JsCast>(value: &str) -> E {
	document()
	.create_element(value)
	.unwrap()
	.dyn_into()
	.unwrap()
}


pub fn console_container() -> Element {
	document().get_element_by_id("console-lines").unwrap()
}

pub fn console_input() -> HtmlInputElement {
	document()
	.get_element_by_id("console-input")
	.unwrap()
	.dyn_into()
	.unwrap()
}


pub fn register_display() {
	{ // On Key Enter Event
		let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
			// Did not press enter.
			if e.key_code() != 13 { return; }

			let line_container = console_input();

			let value = line_container.value();
			line_container.set_value("");

			display_parsed(&value);
		}) as Box<dyn FnMut(_)>);

		console_input().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
		closure.forget();
	}

	help_command();
}

pub fn display_parsed(eval: &str) {
	let mut parser = Parser::new(eval);

	match parser.parse() {
		Ok(v) => {
			// Steps
			log!("Steps:");
			log!(" - {:?}", parser.parsed_tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(""));
			for step in parser.steps {
				log!(" - {:?}", step.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(""));
			}


			log!("Value: {:?}", v);

			let mut tokens = v.into_tokens();

			log!("Value Tokens: {:?}", tokens);


			let mut full = parser.parsed_tokens.clone();
			full.push(Operator::Equal.into());
			full.append(&mut tokens);

			let line = Line::new(full.as_slice());

			console_container().append_child(&line.render());
		},
		Err(e) => log!("{:?}", e)
	}
}

pub fn help_command() {
	let mul_1 = Tokenizer::new("2014 / 2 * 5").parse().unwrap();
	let min_sec = Tokenizer::new("5 min + 30s").parse().unwrap();
	let wrapped = Tokenizer::new("5 * (10 / 2)").parse().unwrap();

	let rows = &[
		mul_1.as_slice(), min_sec.as_slice(),
		wrapped.as_slice()
	];

	let table = Table::new(rows);

	console_container().append_child(&table.render());
}