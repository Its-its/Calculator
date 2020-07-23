// https://en.wikipedia.org/wiki/Orders_of_magnitude_(data)

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{create_non_standard_unit, create_standard_unit};
use super::BaseUnit;


create_standard_unit!(Byte, "byte", "bytes", "B");

create_non_standard_unit!(Bit, Byte, dec!(0.125), "bit", "bits", "bit");
create_non_standard_unit!(KiloByte, Byte, dec!(1024.0), "kilobyte", "kilobytes", "kB");
create_non_standard_unit!(MegaByte, Byte, dec!(1024.0e3), "megabyte", "megabytes", "MB");
create_non_standard_unit!(GigaByte, Byte, dec!(1024.0e6), "gigabyte", "gigabytes", "GB");
create_non_standard_unit!(TeraByte, Byte, dec!(1024.0e9), "terabyte", "terabytes", "TB");
create_non_standard_unit!(PetaByte, Byte, dec!(1024.0e12), "petabyte", "petabytes", "PB");
create_non_standard_unit!(ExaByte, Byte, dec!(1024.0e15), "exabyte", "exabytes", "EB");
create_non_standard_unit!(ZettaByte, Byte, dec!(1024.0e18), "zettabyte", "zettabytes", "ZB");

// Error: attempt to multiply with overflowrustc
// create_non_standard_unit!(YottaByte, Byte, dec!(1024.0e21), "yottabyte", "yottabytes", "YB");