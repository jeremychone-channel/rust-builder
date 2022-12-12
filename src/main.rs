#![allow(unused)] // For starter.

use prelude::*;

mod error;
mod prelude;

fn main() -> Result<()> {
	println!(
		r#"
The code examples are examples/. Run the following: 

cargo run --example builder-non-consuming
cargo run --example builder-consuming
cargo run --example builder-states
	"#
	);

	Ok(())
}
