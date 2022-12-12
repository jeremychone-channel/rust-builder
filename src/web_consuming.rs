//! Consuming builder pattern example.
//! See video: https://youtu.be/Z_3WOSiYYFY
//!

use crate::prelude::*;

#[derive(Debug)]
pub struct Request {
	url: String,
	method: String,                 // Should eventually be an enum
	headers: Vec<(String, String)>, // (name, value)
	body: Option<String>,
}

#[derive(Default, Clone)]
pub struct RequestBuilder {
	url: Option<String>,
	method: Option<String>,
	headers: Vec<(String, String)>,
	body: Option<String>,
}

impl RequestBuilder {
	pub fn new() -> Self {
		RequestBuilder::default()
	}

	pub fn url(mut self, url: impl Into<String>) -> Self {
		self.url.insert(url.into());
		self
	}
	pub fn method(mut self, method: impl Into<String>) -> Self {
		self.method.insert(method.into());
		self
	}
	pub fn body(mut self, body: impl Into<String>) -> Self {
		self.body.insert(body.into());
		self
	}
	pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
		self.headers.push((name.into(), value.into()));
		self
	}

	pub fn build(self) -> Result<Request> {
		// Note: Runtime check if the url is present.
		//       See the state builder pattern to do it a compile time.
		let Some(url) = self.url.as_ref() else {
			return Err(Error::Static("No URL"));
		};

		// Note: In this example, foro method, we will get a default "GET" if none.
		//       In the state builder we will make this required as well to show multi-states.
		let method = self.method.clone().unwrap_or_else(|| "GET".to_string());

		Ok(Request {
			url: url.to_string(),
			method,
			headers: self.headers.clone(),
			body: self.body.clone(),
		})
	}
}
