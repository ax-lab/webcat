mod request_tests {
	use tux::warp;
	use tux::warp::Filter;
	use tux::*;

	fn spawn_test_server() -> TestServer {
		let a = warp::path("a").and(warp::path::end()).map(|| "route a");
		let b = warp::path("b").and(warp::path::end()).map(|| "route b");
		let root = warp::path::end().map(|| "get test root");
		let routes = warp::get().and(root.or(a).or(b));
		TestServer::new_with_routes(routes)
	}

	#[test]
	fn should_run_simple_script() {
		let server = spawn_test_server();

		let data = TestDataDir::create_new();
		data.create_file(
			"simple_request.txt",
			format!("GET localhost:{}", server.port()),
		);

		let output = data.run_and_get_output("webcat", &["simple_request.txt"]);
		assert_eq!(output.trim(), "get test root");
	}

	#[test]
	fn should_run_multiple_scripts() {
		let server = spawn_test_server();

		let data = TestDataDir::create_new();
		data.create_file(
			"request_a.txt",
			format!("GET localhost:{}/a", server.port()),
		);
		data.create_file(
			"request_b.txt",
			format!("GET localhost:{}/b", server.port()),
		);

		let output = data.run_and_get_output("webcat", &["request_a.txt", "request_b.txt"]);
		assert_eq!(output.trim(), "route a\nroute b");
	}
}
