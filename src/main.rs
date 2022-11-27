#![allow(unused)]
use prelude::*;
use task::Task;
use web::{Request, RequestBuilder};
mod error;
mod prelude;
mod task;
mod utils;
mod web;

fn main() -> Result<()> {
	let mut req_builder = RequestBuilder::new();
	req_builder
		.url("https://some-url.com/task/123")
		.method("GET");
	// ... do some stuff
	let req_builder = req_builder.header("token", "user_uuid.exp.sign");
	// ... do other stuff
	let req = req_builder.clone().build()?;
	println!("{req:#?}");

	let req = req_builder.header("Client-Version", "1.2").build()?;
	println!("{req:#?}");

	Ok(())
}
