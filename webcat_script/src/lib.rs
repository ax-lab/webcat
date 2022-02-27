use webcat_web as web;

pub fn run_script_to_string<S: AsRef<str>>(source: S) -> String {
	let source = source.as_ref();
	let source = source.trim().strip_prefix("GET ").unwrap();
	web::Request::new()
		.send(web::RequestMethod::GET, source)
		.unwrap()
		.text()
}

#[cfg(test)]
mod tests {
	use super::*;
	use tux::*;

	#[test]
	fn script_with_a_simple_get_should_work() {
		let server = TestServer::new_with_root_response("simple get worked!");
		let source = format!(
			r##"
				GET localhost:{}
			"##,
			server.port()
		);

		let output = run_script_to_string(source);
		assert_eq!(output, "simple get worked!");
	}

	#[test]
	fn script_with_a_simple_get_should_output_response() {
		let server = TestServer::new_with_root_response("response 1");
		let source = format!(
			r##"
				GET localhost:{}
			"##,
			server.port()
		);
		let output = run_script_to_string(source);
		assert_eq!(output, "response 1");

		let server = TestServer::new_with_root_response("response 2");
		let source = format!(
			r##"
				GET localhost:{}
			"##,
			server.port()
		);
		let output = run_script_to_string(source);
		assert_eq!(output, "response 2");
	}
}
