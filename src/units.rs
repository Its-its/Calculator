
use crate::{Value, Result, Error};
use conversion::units::*;


pub fn default_units() -> Vec<Box<dyn BaseUnit>> {
	vec![
		// TIME
		Box::new(Second),
		Box::new(NanoSecond),
		Box::new(MicroSecond),
		Box::new(MilliSecond),
		Box::new(Minute),
		Box::new(Hour),
		Box::new(Day),
		Box::new(Week),
		Box::new(Fortnight),
		Box::new(Month),
		Box::new(Year),
		Box::new(CommonYear),
		Box::new(Decade),
		Box::new(Century),

		// LENGTH
		Box::new(Meter),

		Box::new(Inch),
		Box::new(Feet),
		Box::new(Yard),
		Box::new(Mile),
		Box::new(NauticalMile)
	]
}

pub fn find_unit(unit: &Box<dyn BaseUnit>) -> Box<dyn BaseUnit> {
	default_units()
	.into_iter()
	.find(|u| u == unit)
	.unwrap()
}

pub fn get_unit_from_literal(name: &str) -> Option<Box<dyn BaseUnit>> {
	default_units()
	.into_iter()
	.find(|u| {
		u.long() == name ||
		u.short().map(|i| i == name).unwrap_or_default() ||
		u.alt().map(|i| i == name).unwrap_or_default()
	})
}

pub fn convert(from: &Value, to: &Value) -> Result<f64> {
	// TODO: Currently will error if doing: 1 -> ms
	let from_unit = from.as_base_unit().ok_or(Error::Text("Cannot convert something that doesn't have a unit".into()))?;
	let to_unit = match to.as_base_unit() {
		Some(u) => u,
		None => return from.amount().ok_or(Error::Text("Base Conversion isn't a number.".into()))
	};

	if is_convertable(from_unit.as_ref(), to_unit.as_ref()) {
		let val = from.amount().unwrap();

		Ok(val * from_unit.base_factor() / to_unit.base_factor())
	} else {
		Err(format!(r#"Values of type "{}" and "{}" are not able to be compaired or converted."#, from_unit.long(), to_unit.long()).into())
	}
}