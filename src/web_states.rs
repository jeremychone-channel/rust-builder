//! State builder pattern example.
//! YouTube video coming...
//!
use crate::prelude::*;

#[derive(Debug)]
pub struct Request {
	url: String,
	method: String,                 // should be enum
	headers: Vec<(String, String)>, // (name, value)
	body: Option<String>,
}

#[derive(Default, Clone)]
pub struct RequestBuilder<U, M> {
	url: U,    // required for .build()
	method: M, // required for .build()
	headers: Vec<(String, String)>,
	body: Option<String>,
}

// region:    --- States
#[derive(Clone)]
pub struct Url(String);
#[derive(Clone)]
pub struct NoUrl;

#[derive(Clone)]
pub struct Method(String);
#[derive(Clone)]
pub struct NoMethod;
// endregion: --- States

impl RequestBuilder<NoUrl, NoMethod> {
	pub fn new() -> Self {
		RequestBuilder {
			url: NoUrl,
			method: NoMethod,
			headers: Vec::new(),
			body: None,
		}
	}
}

impl RequestBuilder<Url, Method> {
	pub fn build(self) -> Result<Request> {
		Ok(Request {
			url: self.url.0,
			method: self.method.0,
			headers: self.headers,
			body: self.body,
		})
	}
}

/// Note: Since we implement url() only when NoUrl,
/// it won't be available once set the url.
impl<M> RequestBuilder<NoUrl, M> {
	pub fn url(mut self, url: impl Into<String>) -> RequestBuilder<Url, M> {
		RequestBuilder {
			url: Url(url.into()),
			method: self.method,
			headers: self.headers,
			body: self.body,
		}
	}
}

/// Note: For Method, we want to allow to reset again, so, we keep
///       the M open as type of the struct, which could be monomorphized in both types.
impl<U, M> RequestBuilder<U, M> {
	pub fn method(
		mut self,
		method: impl Into<String>,
	) -> RequestBuilder<U, Method> {
		RequestBuilder {
			url: self.url,
			method: Method(method.into()),
			headers: self.headers,
			body: self.body,
		}
	}
}

/// Note: `body(...)` and `header(...)` will be allowed for all builder "states."
impl<U, M> RequestBuilder<U, M> {
	pub fn body(mut self, body: impl Into<String>) -> Self {
		self.body.insert(body.into());
		self
	}
	pub fn header(
		mut self,
		name: impl Into<String>,
		value: impl Into<String>,
	) -> Self {
		self.headers
			.push((name.into(), value.into()));
		self
	}
}
