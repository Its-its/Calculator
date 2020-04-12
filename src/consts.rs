// Constants.
// PI, E, etc..

pub type DefaultConsts<'a> = Vec<(&'a str, f64)>;

pub const PI: f64 = std::f64::consts::PI;
pub const EulersNumber: f64 = std::f64::consts::E;


pub fn default_constants<'a>() -> DefaultConsts<'a> {
	vec![
		("PI", PI),
		("E", EulersNumber)
	]
}