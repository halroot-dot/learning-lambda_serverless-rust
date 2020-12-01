use env_logger;
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{debug, error, info, warn};
use rusoto_core::Region;
use serde_derive::{Deserialize, Serialize};
use serde_dynamodb;
use serde_json::{map::ValuesMut, Value};
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::error::Error;

use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};

#[derive(Serialize, Deserialize, Debug, Default)]
struct CustomEvent {
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct CustomOutput {
    id: String,
    first_name: String,
    last_name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    println!("Hello World");
    lambda!(handler);
    Ok(())
}

fn handler(event: CustomEvent, ctx: Context) -> Result<CustomOutput, HandlerError> {
    println!("input: CustomEvent is {:?}", event);
    let mut query_key: HashMap<String, AttributeValue> = HashMap::new();
    query_key.insert(
        "id".to_string(),
        AttributeValue {
            s: Some(event.id),
            ..Default::default()
        },
    );

    let input: GetItemInput = GetItemInput {
        table_name: "learning_lambda_rust".to_string(),
        key: query_key,
        ..Default::default()
    };

    let client = DynamoDbClient::new(Region::ApNortheast1);
    match client.get_item(input).sync() {
        Ok(result) => match result.item {
            Some(item) => {
                println!("item is {:?}", item);

                Ok(serde_dynamodb::from_hashmap(item).unwrap())
            }
            None => {
                error!("{}", "no item was found.");
                Ok(Default::default())
            }
        },
        Err(error) => Err(ctx.new_error(error.description())),
    }
}
