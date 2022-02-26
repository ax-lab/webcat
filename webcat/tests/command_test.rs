mod common;

mod command_tests {
	use super::common::*;

	#[test]
	fn it_should_run() {
		let output = run_and_get_output("webcat");
		assert!(output.contains("webcat"));
	}
}
