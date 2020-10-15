// https://en.wikipedia.org/wiki/Metre

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::create_non_standard_unit;
use super::{BaseUnit, Meter};

// Metric

create_non_standard_unit!(PicoMeter, Meter, dec!(1e-12), "picometer", "picometers", "pm");
create_non_standard_unit!(NanoMeter, Meter, dec!(1e-9), "nanometer", "nanometers", "nm");
create_non_standard_unit!(MicroMeter, Meter, dec!(1e-6), "micrometer", "micrometers", "µm");
create_non_standard_unit!(MilliMeter, Meter, dec!(1e-3), "millimeter", "milliometers", "mm");
create_non_standard_unit!(CentiMeter, Meter, dec!(1e-2), "centimeter", "centimeters", "cm");
create_non_standard_unit!(DeciMeter, Meter, dec!(1e-1), "decimeter", "decimeters", "dm");
// Meter (1)
create_non_standard_unit!(DecaMeter, Meter, dec!(1e+1), "decameter", "decameters", "dam");
create_non_standard_unit!(HectoMeter, Meter, dec!(1e+2), "hectometer", "hectometers", "hm");
create_non_standard_unit!(KiloMeter, Meter, dec!(1e+3), "kilometer", "kilometers", "km");
create_non_standard_unit!(MegaMeter, Meter, dec!(1e+6), "megameter", "megameters", "Mm");
create_non_standard_unit!(GigaMeter, Meter, dec!(1e+9), "gigameter", "gigameters", "Gm");
create_non_standard_unit!(TeraMeter, Meter, dec!(1e+12), "terameter", "terameters", "Tm");


// Imperial

create_non_standard_unit!(Inch, Meter, dec!(0.0254), "inch", "inches", "in", [r#"""#, "″"]);
create_non_standard_unit!(Feet, Meter, dec!(0.3048), "foot", "feet", "ft", ["feet", "'", "′"]);
create_non_standard_unit!(Yard, Meter, dec!(0.9144), "yard", "yards", "yd");
create_non_standard_unit!(Mile, Meter, dec!(1609.34), "mile", "miles", "mile");
create_non_standard_unit!(NauticalMile, Meter, dec!(1852.0), "nautical mile", "nautical mile", "M");