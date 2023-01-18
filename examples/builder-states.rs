#![allow(unused)] // For beginning only.

use rust_base::prelude::*;
use rust_base::web_states::{Request, RequestBuilder, Url};

fn main() -> Result<()> {
	let req_builder = RequestBuilder::new()
		.url("https://some-url.com/")
		.method("GET");

	let req_builder = req_builder
		.header("Token", "uuid.exp.sign")
		.seal();

	let req = req_builder.clone().build()?;
	println!("{req:#?}");

	let req = req_builder.build()?;
	println!("{req:#?}");

	Ok(())
}
