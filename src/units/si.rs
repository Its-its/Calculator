// https://en.wikipedia.org/wiki/International_System_of_Units
// https://en.wikipedia.org/wiki/SI_derived_unit
// https://en.wikipedia.org/wiki/SI_base_unit

use crate::create_standard_unit;
use super::BaseUnit;

// match_conv(typeName, [SECOND = 1.0, ])



create_standard_unit!(Second, "second", "seconds", "s");
create_standard_unit!(Meter, "meter", "meters", "m");
create_standard_unit!(Gram, "gram", "grams", "g");
create_standard_unit!(Kilogram, "kilogram", "kilograms", "kg");
create_standard_unit!(Ampere, "ampere", "amperes", "A");
create_standard_unit!(Mole, "mole", "moles", "mol");
create_standard_unit!(Kelvin, "kelvin", "kelvins", "K");
create_standard_unit!(Candela, "candela", "candelas", "cd");