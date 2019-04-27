use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
	let profile = env::var("PROFILE").unwrap();
	let mapping = format!("/background.wasm => target/wasm32-unknown-unknown/{}/background.wasm", &profile);
	fs::write("mappings.sb", mapping)
}
