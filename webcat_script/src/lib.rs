use webcat_web as web;

pub fn run_script_to_string<S: AsRef<str>>(source: S) -> String {
	let source = source.as_ref();
	let source = source.trim();
	if source.len() == 0 {
		return String::new();
	}

	let (method, source) = if let Some(source) = source.strip_prefix("GET ") {
		(web::RequestMethod::GET, source)
	} else if let Some(source) = source.strip_prefix("POST ") {
		(web::RequestMethod::POST, source)
	} else {
		panic!("invalid method");
	};
	web::Request::new().send(method, source).unwrap().text()
}

#[cfg(test)]
mod tests {
	use super::*;
	use tux::*;

	#[test]
	fn empty_script_has_empty_output() {
		let output = run_script_to_string("");
		assert_eq!(output, "");
	}

	#[test]
	fn script_supports_get_request() {
		let server = TestServer::new_with_root_response("simple get worked!");
		let source = format!("GET localhost:{}", server.port());

		let output = run_script_to_string(source);
		assert_eq!(output, "simple get worked!");
	}

	#[test]
	fn script_with_a_single_request_outputs_response() {
		let server = TestServer::new_with_root_response("response 1");
		let source = format!("GET localhost:{}", server.port());
		let output = run_script_to_string(source);
		assert_eq!(output, "response 1");

		let server = TestServer::new_with_root_response("response 2");
		let source = format!("GET localhost:{}", server.port());
		let output = run_script_to_string(source);
		assert_eq!(output, "response 2");
	}

	#[test]
	fn script_supports_post_request() {
		let server = TestServer::new_with_ping_route("ping");
		let output = helper::run_test_script_at_server(server, "POST localhost:0000/ping");
		assert!(output.contains("method: POST"));
	}

	mod helper {
		use super::*;

		pub fn run_test_script_at_server(server: TestServer, script: &str) -> String {
			let port = format!(":{}", server.port());
			let script = script.replace(":0000", &port);
			let output = run_script_to_string(script);
			output
		}
	}
}
