use conversion::{BaseUnit, FunctionEval};

use crate::{Parser, ParseValue, functions, units, consts, Value, Result};


pub struct Factory {
	functions: Vec<(String, Box<dyn FunctionEval>)>,
	units: Vec<Box<dyn BaseUnit>>,
	consts: Vec<(String, f64)>
}

impl Factory {
	pub fn new() -> Self {
		Self {
			functions: functions::default_functions().into_iter().map(|f| (f.0.to_string(), f.1)).collect(),
			consts: consts::default_constants().into_iter().map(|f| (f.0.to_string(), f.1)).collect(),
			units: units::default_units(),
		}
	}


	pub fn create_parser<'a>(&'a self, eval: &'a str) -> Parser<'a> {
		Parser::new(self, eval)
	}

	pub fn parse(&self, eval: &str) -> Result<ParseValue> {
		self.create_parser(eval).parse()
	}


	pub fn find_const(&self, name: &str) -> Option<f64> {
		self.consts
		.iter()
		.find(|u| u.0 == name)
		.map(|i| i.1)
	}


	pub fn find_func(&self, name: &str) -> Option<Box<dyn FunctionEval>> {
		self.functions
		.iter()
		.find(|u| u.0 == name)
		.map(|i| i.1.clone())
	}

	pub fn find_unit(&self, name: &str) -> Box<dyn BaseUnit> {
		self.units
		.iter()
		.find(|u| u.as_ref() == &name)
		.map(|i| i.clone())
		.unwrap_or_else(|| Box::new(units::CustomUnit::new(name.to_string())))
	}

	pub fn is_custom_unit(&self, name: &str) -> bool {
		self.units
		.iter()
		.find(|u| u.as_ref() == &name)
		.is_none()
	}
}