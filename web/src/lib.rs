#![warn(warnings, rust_2018_idioms, unsafe_code, dead_code)]
#![warn(clippy::all)]
#![allow(clippy::missing_docs_in_private_items)]

use wasm_bindgen::prelude::*;


#[macro_use]
macro_rules! log {
	($($arg:tt)*) => {{
		crate::log_1(&format!($($arg)*));
	}};
}


#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console, js_name=log)]
	fn log_1(s: &str);
}


pub mod console;
pub mod display;

pub use display::{Line, LineDisplay, Table};
pub use console::{console_container, console_input, document, window, create_element};


#[wasm_bindgen(start)]
pub fn run() {
	console_error_panic_hook::set_once();

	log!("{}", display::pretty_print("1000000.1234543".chars()));

	log!("testing");

	console::register_display();
}