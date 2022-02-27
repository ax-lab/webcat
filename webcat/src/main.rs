use webcat_script as script;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 1 {
		println!("Usage: webcat SCRIPT...");
	} else {
		for file in args.iter().skip(1) {
			let script = std::fs::read_to_string(file).expect("could not open input file");
			let output = script::run_script_to_string(script);
			println!("{}", output);
		}
	}
}
