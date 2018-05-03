extern crate serde;
#[macro_use]
extern crate serde_json;

use std::env;

fn main() {
	let mut args = vec![];
	for arg in env::args() {
		args.push(arg);
	}
	let json = json!({
		"message": "Hello, world!",
		"args": json!(args),
	});
	println!("{}", serde_json::to_string(&json).unwrap());
}
