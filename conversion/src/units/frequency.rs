// https://en.wikipedia.org/wiki/Hertz
// https://en.wikipedia.org/wiki/Orders_of_magnitude_(frequency)

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::create_non_standard_unit;
use super::{BaseUnit, Hertz};

// Metric

create_non_standard_unit!(PicoHertz, Hertz, dec!(1e-12), "picohertz", "picohertz", "pHz");
create_non_standard_unit!(NanoHertz, Hertz, dec!(1e-9), "nanohertz", "nanohertz", "nHz");
create_non_standard_unit!(MicroHertz, Hertz, dec!(1e-6), "microhertz", "microhertz", "µHz");
create_non_standard_unit!(MilliHertz, Hertz, dec!(1e-3), "millihertz", "milliohertz", "mHz");
create_non_standard_unit!(CentiHertz, Hertz, dec!(1e-2), "centihertz", "centihertz", "cHz");
create_non_standard_unit!(DeciHertz, Hertz, dec!(1e-1), "decihertz", "decihertz", "dHz");
// Hertz (1)
create_non_standard_unit!(DecaHertz, Hertz, dec!(1e+1), "decahertz", "decahertz", "daHz");
create_non_standard_unit!(HectoHertz, Hertz, dec!(1e+2), "hectohertz", "hectohertz", "hHz");
create_non_standard_unit!(KiloHertz, Hertz, dec!(1e+3), "kilohertz", "kilohertz", "kHz");
create_non_standard_unit!(MegaHertz, Hertz, dec!(1e+6), "megahertz", "megahertz", "MHz");
create_non_standard_unit!(GigaHertz, Hertz, dec!(1e+9), "gigahertz", "gigahertz", "GHz");
create_non_standard_unit!(TeraHertz, Hertz, dec!(1e+12), "terahertz", "terahertz", "THz");