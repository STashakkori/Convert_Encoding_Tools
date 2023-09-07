use std::env;
use to_binary::BinaryString;
fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 || args.len() > 2 {
		println!("  USAGE: ");
		println!("  ./string2bin  'input encased in single quotes' ");
		return;
	}
	
	let stringtext = args[1].trim().to_string();
	let bin_string = BinaryString::from(stringtext.clone());
	let bin_with_spaces = match bin_string.add_spaces() {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to add spaces : {:?}", err);
			return;
		}	
	};
	println!("{}",bin_with_spaces.to_string());
}
