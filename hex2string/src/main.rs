use std::env;
use core::str;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 || args.len() > 2 {
		println!("  USAGE: ");
		println!("  ./hex2string  'input encased in single quotes' ");
		return;
	}
	
	let stringtext = args[1].trim().to_string();
	let stringtexttrim = stringtext.trim().replace(" ", "");
	if !(stringtexttrim.len() % 2 == 0) {
		println!("Please enter valid hex input divisible by 2. [length: {}]", stringtexttrim.len());
		return;
	}
	
	let buffer = match hex::decode(stringtexttrim) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to decode, Error : {}", err);
			return;
		}	
	};
	let strings = match str::from_utf8(&buffer) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to parse, Error : {}", err);
			return;
		}	
	};

	println!("{}", strings);
}


