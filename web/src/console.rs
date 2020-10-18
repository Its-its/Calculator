use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

use conversion_parser::{Factory, Operator, ExprToken, Tokenizer};

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
	let factory = Factory::new();

	let mut parser = factory.create_parser(eval);

	match parser.parse() {
		Ok(v) => {
			if parser.get_parsed_tokens().len() == 1 {
				if let Some(token) = parser.get_parsed_tokens().first().cloned() {
					if token.is_literal() {
						let t = &[token.clone()];
						let line = Line::new(t);

						let _ = console_container().append_child(&line.render());


						let command_name = token.into_literal();

						match command_name.as_str() {
							"help" => help_command(),
							"functions" => fn_list_command(&factory),
							"constants" => const_list_command(&factory),
							"units" => unit_list_command(&factory),
							_ => {}
						}

						return;
					}
				}
			}

			// Steps
			log!("Steps:");
			log!(" - {:?}", parser.get_parsed_tokens().iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(""));
			for step in parser.steps.as_slice() {
				log!(" - {:?}", step.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(""));
			}


			log!("Value: {:?}", v);

			let mut tokens = v.into_tokens();

			log!("Value Tokens: {:?}", tokens);


			let mut full = parser.get_parsed_tokens().to_vec();
			full.push(Operator::Equal.into());
			full.append(&mut tokens);

			let line = Line::new(full.as_slice());

			let _ = console_container().append_child(&line.render());
		},
		Err(e) => log!("{:?}", e)
	}
}


pub fn help_command() {
	let factory = Factory::new();

	let mul_1 = into_tokens("2014 / 2 * 5", &factory);
	let min_sec = into_tokens("5 min 30 s", &factory);

	let wrapped = into_tokens("5 * (10 / 2)", &factory);
	let m_into_h = into_tokens("120 min -> h", &factory);

	let command_help = into_tokens("help", &factory);
	let command_fn = into_tokens("functions", &factory);
	let command_units = into_tokens("units", &factory);
	let command_const = into_tokens("constants", &factory);

	let empty_vec = vec![ExprToken::Whitespace];

	let rows = vec![
		mul_1, min_sec,
		wrapped, m_into_h,

		empty_vec.clone(), empty_vec,

		command_help, command_fn,
		command_units, command_const,
	];

	let table = Table::new(rows);

	let _ = console_container().append_child(&table.render());
}

pub fn unit_list_command(factory: &Factory) {
	let units = factory.get_units();

	let rows: Vec<Vec<ExprToken>> = units.iter()
		.map(|f| vec![
			ExprToken::Literal(f.long().to_string()),
			// ExprToken::Literal(f.short().map(|a| a.to_string()).unwrap_or_default()),
		])
		.collect();

	let table = Table::new(rows);

	let _ = console_container().append_child(&table.render());
}

pub fn const_list_command(factory: &Factory) {
	let constants = factory.get_constants();

	let rows: Vec<Vec<ExprToken>> = constants.iter().map(|f| vec![ExprToken::Literal(f.0.clone())]).collect();

	let table = Table::new(rows);

	let _ = console_container().append_child(&table.render());
}

pub fn fn_list_command(factory: &Factory) {
	let functions = factory.get_functions();

	let rows: Vec<Vec<ExprToken>> = functions.iter().map(|f| vec![ExprToken::Literal(f.0.clone())]).collect();

	let table = Table::new(rows);

	let _ = console_container().append_child(&table.render());
}


fn into_tokens(value: &str, factory: &Factory) -> Vec<ExprToken> {
	let mut token = Tokenizer::new(value, factory);

	token.parse().unwrap();

	token.get_tokens()
}