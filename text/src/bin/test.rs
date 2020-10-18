use conversion_text::{parse, Result};

fn main() -> Result<()> {
	let _ = parse("[Prebuilt] Newegg's ABS Gaming PC - RTX 2070 SUPER, 4 x 120mm Addressable RGB Fans, Wireless AC + Bluetooth, Windows 10 Home, Gaming Mouse + Keyboard, 512 GB SSD, 16 GB DDR4 3000 MHz RAM, Ryzen 5 3600 - $1049 (or $944 for New Members)")?;

	Ok(())
}