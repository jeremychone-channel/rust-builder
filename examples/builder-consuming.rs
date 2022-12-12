//! Consuming builder pattern example.
//! See video: https://youtu.be/Z_3WOSiYYFY
//!

#![allow(unused)]
use rust_base::prelude::*;
use rust_base::web_consuming::{Request, RequestBuilder};

fn main() -> Result<()> {
	// Note: In the consuming pattern, we can start chaining and reuse afer.
	let req_builder = RequestBuilder::new()
		.url("https://some-url.com/task/123")
		.method("GET");

	// ... do some stuff

	// Note: But the builder needs to be reassign until final use.
	let req_builder = req_builder.header("token", "user_uuid.exp.sign");

	// ... do other stuff

	// Note: And clone needs to be explicit (for the better)
	let req = req_builder.clone().build()?;
	println!("{req:#?}");

	// Builder reuse (ok, as original was cloned before)
	let req = req_builder
		.header("Client-Version", "1.2")
		.method("POST")
		.body(r#"{"test": "hello"}"#)
		.build()?;

	println!("{req:#?}");

	Ok(())
}
