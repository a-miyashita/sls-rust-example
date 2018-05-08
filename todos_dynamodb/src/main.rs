extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use std::env;
use std::process;
use std::collections::HashMap;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, AttributeValue};

const USAGE: &'static str = "
Usage:
	todos_dynamodb GET <path> [--id <id>]

Options:
	--id <id>  todo id
";

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Args {
	cmd_GET: bool,
	arg_path: String,
	flag_id: String,
}

fn query_by_id(id: &str) -> GetItemInput {
	let mut av: AttributeValue = Default::default();
	av.s = Some(id.to_string());
	let mut key = HashMap::new();
	key.insert("id".to_string(), av);
	GetItemInput {
		table_name: format!("{}-todos", env::var("STAGE").unwrap_or("dev".to_string())),
		attributes_to_get: Some(vec!["id".to_string(), "message".to_string()]),
		consistent_read: None,
		expression_attribute_names: None,
		key: key,
		projection_expression: None,
		return_consumed_capacity: None,
	}
}

fn main() {
	let args: Args = docopt::Docopt::new(USAGE)
		.and_then(|d| d.deserialize())
		.unwrap_or_else(|e| {
			println!("{:?}", serde_json::to_string(&json!({
				"message": format!("{:?}", &e),
			})).unwrap());
			process::exit(204); // 404 not found
		});
	let client = DynamoDbClient::simple(Region::default());
	match client.get_item(&query_by_id(&args.flag_id)).sync() {
		Ok(result) => {
			if let Some(todo) = result.item {
				println!("{}", serde_json::to_string(&todo).unwrap());
				process::exit(0);
			} else {
				process::exit(204); // 404 not found
			}
		},
		Err(e) => {
			println!("{}", serde_json::to_string(&json!({
				"message": format!("{:?}", &e),
			})).unwrap());
			process::exit(204); // 404 not found
		},
	}
}
