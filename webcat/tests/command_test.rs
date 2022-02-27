mod command_tests {
	use tux::*;

	#[test]
	fn it_should_run() {
		let output = run_and_get_output("webcat");
		assert!(output.contains("webcat"));
	}
}
