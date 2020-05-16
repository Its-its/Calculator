use conversion::{BaseUnit, FunctionEval};

use crate::{Parser, functions, units, Value, Result};


pub struct Factory {
	functions: Vec<(String, Box<dyn FunctionEval>)>,
	units: Vec<Box<dyn BaseUnit>>
}

impl Factory {
	pub fn new() -> Self {
		Self {
			functions: functions::default_functions().into_iter().map(|f| (f.0.to_string(), f.1)).collect(),
			units: units::default_units()
		}
	}


	pub fn create_parser<'a>(&'a self, eval: &'a str) -> Parser<'a> {
		Parser::new(self, eval)
	}

	pub fn parse(&self, eval: &str) -> Result<Value> {
		self.create_parser(eval).parse()
	}


	pub fn find_func(&self, name: &str) -> Option<Box<dyn FunctionEval>> {
		self.functions
		.iter()
		.find(|u| u.0 == name)
		.map(|i| i.1.clone())
	}

	pub fn find_unit(&self, name: &str) -> Option<Box<dyn BaseUnit>> {
		self.units
		.iter()
		.find(|u| {
			u.long() == name ||
			u.multiple() == name ||
			u.short().map(|i| i == name).unwrap_or_default() ||
			u.alt().into_iter().find(|i| i == &name).is_some()
		})
		.map(|i| i.clone())
	}
}