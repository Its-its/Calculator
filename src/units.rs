use conversion::units::*;


pub fn default_units() -> Vec<Box<dyn BaseUnit>> {
	vec![
		// TIME
		Box::new(SECOND),
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

		//
	]
}

pub fn find_unit(unit: &Box<dyn BaseUnit>) -> Box<dyn BaseUnit> {
	default_units()
	.into_iter()
	.find(|u| u == unit)
	.unwrap()
}