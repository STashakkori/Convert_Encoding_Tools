use std::env;
fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 || args.len() > 2 {
		println!("  USAGE: ");
		println!("  ./string2hex  'input encased in single quotes' ");
		return;
	}
	
	let stringtext = args[1].trim().to_string();
	let mut hex_string = hex::encode(stringtext.clone());
	let mut ick = 0;	
	
	for i in 0..hex_string.len() {
		if i % 2 == 0 && i > 0 {
			hex_string.insert(ick,' ');
			ick+=1;
		}
		ick +=1;
	}

	println!("{}",hex_string);
}
