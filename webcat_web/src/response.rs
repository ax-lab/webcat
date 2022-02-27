pub struct Response {
	status: u16,
	payload: String,
}

impl Response {
	pub(crate) fn from_reqwest(response: reqwest::blocking::Response) -> Self {
		let status = response.status().as_u16();
		let payload = response.bytes().expect("reading response bytes");
		let payload = String::from_utf8_lossy(&payload).to_string();
		Response { status, payload }
	}

	pub fn text(&self) -> String {
		self.payload.clone()
	}

	pub fn status_code(&self) -> u16 {
		self.status
	}
}
