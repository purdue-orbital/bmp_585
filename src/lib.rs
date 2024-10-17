// look at this page for docs https://docs.rs/embedded-hal/latest/embedded_hal/

use embedded_hal::i2c::I2c;

struct BMP585 {

}

pub fn add(left: u64, right: u64) -> u64 {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
