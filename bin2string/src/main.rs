// Binary To String Conversion
// Courtesy of QVLx Labs

use std::env;
use std::{num::ParseIntError};

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 || args.len() > 2 {
		println!("  USAGE: ");
		println!("  ./bin2string  'input encased in single quotes' ");
		return;
	}
	let mut ick = 0;
	let mut stringtext = args[1].trim().to_string();
	let nospace = stringtext.replace(" ","");
	if !(nospace.len() % 8 == 0) {
		println!("Please enter input divisible by 8. [length: {}]", nospace.len());
		return;
	}
	
	
	if !stringtext.contains(' ') {	
		for i in 0..stringtext.len() {
			if i % 8 == 0 && i > 0 {
				stringtext.insert(ick,' ');
				ick+=1;
			}
			ick +=1;
		}
	}
	
	let input = match decode_binary(&stringtext) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to decode, Error : {}", err);
			return;
		}	
	};
	let inputdone = match String::from_utf8(input) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to decode, Error : {}", err);
			return;
		}	
	};
	
	println!("{}", inputdone);
}

pub fn decode_binary(s: &str) -> Result<Vec<u8>, ParseIntError> {
  (0..s.len())
	.step_by(9)
	.map(|i| u8::from_str_radix(&s[i..i + 8], 2))
	.collect()
}
