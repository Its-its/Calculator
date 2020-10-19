use conversion_text::{parse, Result};
use conversion_parser::Factory;
use conversion::{GigaByte, Quantity, Units};

fn main() -> Result<()> {
	let parsed = parse("[Prebuilt] Newegg's ABS Gaming PC - RTX 2070 SUPER, 4 x 120mm Addressable RGB Fans, Wireless AC + Bluetooth, Windows 10 Home, Gaming Mouse + Keyboard, 512 GB SSD, 16 GB DDR4 3000 MHz RAM, Ryzen 5 3600 - $1049 (or $944 for New Members)", Factory::new())?;

	// Examples:
	// - matcher_for(GigaByte).quantity_unit(100.0, GigaByte).find_greater_than().exists()
	// - matcher_for(GigaByte).quantity_unit(100.0, GigaByte).label(label)

	println!("{:?}", parsed.greater_than(GigaByte, (100.0, GigaByte)));


	Ok(())
}