use crate::prelude::*;

#[derive(Debug)]
pub struct Request {
	url: String,
	method: String,                 // should be enum
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

	pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
		self.url.insert(url.into());
		self
	}
	pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
		self.method.insert(method.into());
		self
	}
	pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
		self.body.insert(body.into());
		self
	}
	pub fn header(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
		self.headers.push((name.into(), value.into()));
		self
	}

	pub fn build(&self) -> Result<Request> {
		let Some(url) = self.url.as_ref() else {
			return Err(Error::Static("No URL"));
		};
		let method = self.method.clone().unwrap_or_else(|| "GET".to_string());

		Ok(Request {
			url: url.to_string(),
			method,
			headers: self.headers.clone(),
			body: self.body.clone(),
		})
	}
}
