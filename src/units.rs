
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
		Box::new(NauticalMile),

		// DATA
		Box::new(Byte),
		Box::new(Bit),
		Box::new(KiloByte),
		Box::new(MegaByte),
		Box::new(GigaByte),
		Box::new(TeraByte),
		Box::new(PetaByte),
		Box::new(ExaByte),
		Box::new(ZettaByte),
		Box::new(YottaByte)
	]
}


pub fn convert(from: &Value, to: &Value) -> Result<f64> {
	// TODO: Currently will error if doing: 1 -> ms
	let from_unit = from.as_base_unit().ok_or(Error::Text("Cannot convert something that doesn't have a unit".into()))?;
	let to_unit = match to.as_base_unit() {
		Some(u) => u,
		None => return from.amount().ok_or(Error::Text("Base Conversion isn't a number.".into()))
	};

	if is_convertable(from_unit, to_unit) {
		let mut val = from.amount().unwrap();

		if !from_unit.is_base_equal(to_unit) {
			val = val * from_unit.base().base_factor() / to_unit.base().base_factor();
		}

		if !from_unit.is_base_2_equal(to_unit) {
			let factor_1 = from_unit.base_2().map(|b| b.base_factor()).unwrap_or(1.0);
			let factor_2 = to_unit.base_2().map(|b| b.base_factor()).unwrap_or(1.0);

			val = (val / factor_1) * factor_2;
		}

		Ok(val)
	} else {
		Err(format!(r#"Values of type "{}" and "{}" are not able to be compaired or converted."#, from_unit.long(), to_unit.long()).into())
	}
}



#[derive(Debug, Clone)]
pub struct CustomUnit(String);

impl CustomUnit {
	pub fn new(unit: String) -> Self {
		CustomUnit(unit)
	}
}

impl BaseUnit for CustomUnit {
	fn multiple(&self) -> &str {
		self.0.as_str()
	}

	fn long(&self) -> &str {
		self.0.as_str()
	}

	fn short(&self) -> Option<&str> {
		Some(self.0.as_str())
	}

	fn alt(&self) -> Vec<&str> {
		Vec::new()
	}

	fn base_unit(&self) -> Option<&dyn BaseUnit> {
		None
	}
}