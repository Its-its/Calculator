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