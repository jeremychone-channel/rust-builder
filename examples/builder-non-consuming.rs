//! Non-Consuming builder pattern example.
//! See video: https://youtu.be/Z_3WOSiYYFY

#![allow(unused)]
use rust_base::prelude::*;
use rust_base::web_non_consuming::{Request, RequestBuilder};

fn main() -> Result<()> {
	// Note: In the non-consuming pattern, if multi-step build,
	//       it is required to have
	//       the new() as a single line to hold the owned
	//       req_builder.
	let mut req_builder = RequestBuilder::new();

	req_builder
		.url("https://some-url.com/task/123")
		.method("GET");

	// ... do some stuff

	let req_builder = req_builder.header("token", "user_uuid.exp.sign");

	// ... do other stuff

	let req = req_builder.clone().build()?;
	println!("{req:#?}");

	// Reuse builder for another request.
	let req = req_builder
		.header("Client-Version", "1.2")
		.method("POST")
		.body(r#"{"test": "hello"}"#)
		.build()?;

	println!("{req:#?}");

	Ok(())
}
