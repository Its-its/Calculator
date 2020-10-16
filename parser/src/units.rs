use rust_decimal::Decimal;

use conversion::units::*;

use crate::{Value, Result, Error};
use crate::operations::ExpressionArg;

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
		Box::new(PicoMeter),
		Box::new(NanoMeter),
		Box::new(MicroMeter),
		Box::new(MilliMeter),
		Box::new(CentiMeter),
		Box::new(DeciMeter),
		Box::new(Meter),
		Box::new(DecaMeter),
		Box::new(HectoMeter),
		Box::new(KiloMeter),
		Box::new(MegaMeter),
		Box::new(GigaMeter),
		Box::new(TeraMeter),

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
		// Box::new(YottaByte)

		// MASS
		Box::new(PicoGram),
		Box::new(NanoGram),
		Box::new(MicroGram),
		Box::new(MilliGram),
		Box::new(CentiGram),
		Box::new(DeciGram),
		Box::new(Gram),
		Box::new(DecaGram),
		Box::new(HectoGram),
		Box::new(KiloGram),
		Box::new(MegaGram),
		Box::new(GigaGram),
		Box::new(TeraGram),

		Box::new(Tonne),
		Box::new(KiloTonne),
		Box::new(MegaTonne),
		Box::new(GigaTonne),

		Box::new(Pound),
		Box::new(Ounce),
	]
}


pub fn can_operate(one: &ExpressionArg, two: &ExpressionArg) -> bool {
	let eval_1 = one.eval().unwrap();
	let eval_2 = two.eval().unwrap();

	match (eval_1.into_base_unit(), eval_2.into_base_unit()) {
		(Some(_), None) |
		(None, Some(_)) |
		(None, None) => true,

		(Some(u1), Some(u2)) => u1 == u2
	}
}


pub fn convert(from: &Value, to: &Value) -> Result<Decimal> {
	// TODO: Currently will error if doing: 1 -> ms
	let from_unit = from.as_base_unit().ok_or_else(|| Error::Text("Cannot convert something that doesn't have a unit".into()))?;
	let to_unit = match to.as_base_unit() {
		Some(u) => u,
		None => return from.amount().ok_or_else(|| Error::Text("Base Conversion isn't a number.".into()))
	};

	if is_convertable(from_unit, to_unit) {
		let mut val = from.amount().unwrap();

		if from_unit.is_base_equal(to_unit) {
			val = val * from_unit.base().base_factor() / to_unit.base().base_factor();
		}

		if from_unit.is_base_2_equal(to_unit) {
			let factor_1 = from_unit.base_2().map(|b| b.base_factor()).unwrap_or_else(|| Decimal::new(1, 0));
			let factor_2 = to_unit.base_2().map(|b| b.base_factor()).unwrap_or_else(|| Decimal::new(1, 0));

			val = (val / factor_1) * factor_2;
		}

		Ok(val)
	} else {
		Err(format!(r#"Values of type "{}" and "{}" are not able to be compaired or converted."#, from_unit.long(), to_unit.long()).into())
	}
}