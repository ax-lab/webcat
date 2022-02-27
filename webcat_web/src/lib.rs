use reqwest::Url;

/// Provides support for setting up and executing HTTP requests.
pub struct Request {}

impl Request {
	pub fn new() -> Request {
		Request {}
	}

	pub fn send<T: AsRef<str>>(&self, method: RequestMethod, url: T) -> RequestResult {
		let url = Url::parse(url.as_ref())
			.map_err(|err| RequestError::InvalidConfiguration(err.to_string()))?;
		let client = reqwest::blocking::Client::new();

		let method = match method {
			RequestMethod::GET => reqwest::Method::GET,
			RequestMethod::POST => reqwest::Method::POST,
		};

		let result = client
			.request(method, url)
			.send()
			.map_err(|err| RequestError::ConnectionFailed(err.to_string()))?;
		let status = result.status().as_u16();
		let payload = result.bytes().expect("reading response bytes");
		let payload = String::from_utf8_lossy(&payload).to_string();
		let result = Response { status, payload };
		Ok(result)
	}
}

pub enum RequestMethod {
	GET,
	POST,
}

#[derive(Debug)]
pub enum RequestError {
	ConnectionFailed(String),
	InvalidConfiguration(String),
}

impl std::fmt::Display for RequestError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RequestError::ConnectionFailed(msg) => write!(f, "connection error: {}", msg),
			RequestError::InvalidConfiguration(msg) => write!(f, "invalid request: {}", msg),
		}
	}
}

pub type RequestResult = std::result::Result<Response, RequestError>;

pub struct Response {
	status: u16,
	payload: String,
}

impl Response {
	pub fn text(&self) -> String {
		self.payload.clone()
	}

	pub fn status_code(&self) -> u16 {
		self.status
	}
}

#[cfg(test)]
mod request_tests {
	use super::*;
	use tux::*;

	#[test]
	fn simple_get_should_return_response_body() {
		let server = TestServer::new("test server");
		let url = format!("http://127.0.0.1:{}", server.port());
		let result = Request::new()
			.send(RequestMethod::GET, url)
			.expect("request failed");
		assert_eq!(result.status_code(), 200);
		assert_eq!(result.text(), "test server");
	}

	#[test]
	fn returns_404_for_inexistent_path() {
		let server = TestServer::new("test server");
		let url = format!("http://127.0.0.1:{}/this_does_not_exist", server.port());
		let result = Request::new()
			.send(RequestMethod::GET, url)
			.expect("request failed");
		let result = result.status_code();
		assert_eq!(result, 404);
	}

	#[test]
	fn returns_error_for_inexistent_server() {
		let result = Request::new().send(RequestMethod::GET, "http://127.0.0.1:753");
		match result {
			Err(RequestError::ConnectionFailed(_)) => {}
			Err(err) => {
				panic!("wrong error: {}", err)
			}
			Ok(_) => {
				panic!("did not fail")
			}
		}
	}

	#[test]
	fn returns_error_for_invalid_url() {
		// Try to setup a request with an invalid URL port
		let result = Request::new().send(RequestMethod::GET, "http://127.0.0.1:99999");
		match result {
			Err(RequestError::InvalidConfiguration(_)) => {}
			Err(err) => {
				panic!("wrong error: {}", err)
			}
			Ok(_) => {
				panic!("did not fail")
			}
		}
	}
}
