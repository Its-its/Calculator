use crate::create_non_standard_unit;
use super::{BaseUnit, Meter};

create_non_standard_unit!(Inch, Meter, 0.0254, "inch", "inches", "in", [r#"""#, "″"]);
create_non_standard_unit!(Feet, Meter, 0.3048, "foot", "feet", "ft", ["feet", "'", "′"]);
create_non_standard_unit!(Yard, Meter, 0.9144, "yard", "yards", "yd");
create_non_standard_unit!(Mile, Meter, 1609.34, "mile", "miles", "mile");
create_non_standard_unit!(NauticalMile, Meter, 1852.0, "nautical mile", "nautical mile", "M");