//! State builder pattern example.
//! YouTube video coming...

#![allow(unused)]
use rust_base::prelude::*;
use rust_base::web_states::{Request, RequestBuilder};

fn main() -> Result<()> {
	// Note: State Builder pattern is a variant of the consuming builder pattern
	let req_builder = RequestBuilder::new()
		.url("https://some-url.com/task/123")
		.method("GET");

	// ... do some stuff

	let req_builder = req_builder.header("token", "user_uuid.exp.sign");

	// ... do other stuff

	let req = req_builder.clone().build()?;
	println!("{req:#?}");

	// Builder reuse (ok, as original was cloned before)
	let req = req_builder
		.header("Client-Version", "1.2")
		// .url("https://nice-try.com") // Would not compile as setup by builder states.
		.method("POST")
		.body(r#"{"test": "hello"}"#)
		.build()?;

	println!("{req:#?}");

	Ok(())
}
