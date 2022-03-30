use tux::*;

#[test]
fn webcat_executable_should_run() {
	let output = run_bin("webcat", &[]);
	assert!(output.contains("Usage: webcat"));
}
