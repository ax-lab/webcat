use tux::*;

#[test]
fn webcat_executable_should_run() {
	let output = run_and_get_output("webcat", &[]);
	assert!(output.contains("Usage: webcat"));
}
