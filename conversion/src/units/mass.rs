// https://en.wikipedia.org/wiki/Kilogram


use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::create_non_standard_unit;
use super::{BaseUnit, Gram};

// Metric

create_non_standard_unit!(PicoGram, Gram, dec!(1e-12), "picogram", "picograms", "pg");
create_non_standard_unit!(NanoGram, Gram, dec!(1e-9), "nanogram", "nanograms", "ng");
create_non_standard_unit!(MicroGram, Gram, dec!(1e-6), "microgram", "micrograms", "µg");
create_non_standard_unit!(MilliGram, Gram, dec!(1e-3), "milligram", "milliograms", "mg");
create_non_standard_unit!(CentiGram, Gram, dec!(1e-2), "centigram", "centigrams", "cg");
create_non_standard_unit!(DeciGram, Gram, dec!(1e-1), "decigram", "decigrams", "dg");
// Gram (1)
create_non_standard_unit!(DecaGram, Gram, dec!(1e+1), "decagram", "decagrams", "dag");
create_non_standard_unit!(HectoGram, Gram, dec!(1e+2), "hectogram", "hectograms", "hg");
create_non_standard_unit!(KiloGram, Gram, dec!(1e+3), "kilogram", "kilograms", "kg");
create_non_standard_unit!(MegaGram, Gram, dec!(1e+6), "megagram", "megagrams", "Mg");
create_non_standard_unit!(GigaGram, Gram, dec!(1e+9), "gigagram", "gigagrams", "Gg");
create_non_standard_unit!(TeraGram, Gram, dec!(1e+12), "teragram", "teragrams", "Tg");

// Tonne
create_non_standard_unit!(Tonne, Gram, dec!(1e+6), "tonne", "tonnes", "t");
create_non_standard_unit!(KiloTonne, Gram, dec!(1e+9), "kilotonne", "kilotonnes", "kt");
create_non_standard_unit!(MegaTonne, Gram, dec!(1e+12), "megatonne", "megatonnes", "Mt");
create_non_standard_unit!(GigaTonne, Gram, dec!(1e+15), "gigatonne", "gigatonnes", "Gt");

// Imperial

create_non_standard_unit!(Pound, Gram, dec!(453.59265), "pound", "pounds", "lb");
create_non_standard_unit!(Ounce, Gram, dec!(28.349523125), "ounce", "ounces", "oz");
