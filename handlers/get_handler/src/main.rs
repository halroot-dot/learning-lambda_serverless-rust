use lambda::{handler_fn, Context};
use lambda_runtime::error::HandlerError;
use rusoto_core::Region;

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;

use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(get_handler)).await?;
    Ok(())
}

async fn get_handler(event: CustomEvent, _ctx: Context) -> Result<CustomOutput, HandlerError> {
    println!("input: CustomEvent is {:?}", event);

    let mut query_key = HashMap::new();
    query_key.insert(
        String::from("id"),
        AttributeValue {
            s: Some(event.id),
            ..Default::default()
        },
    );

    let input: GetItemInput = GetItemInput {
        table_name: String::from("learning_lambda_rust"),
        key: query_key,
        ..Default::default()
    };

    let client = DynamoDbClient::new(Region::ApNortheast1);
    match client.get_item(input).await {
        Ok(result) => match result.item {
            Some(item) => {
                println!("item in database: {:?}", item);
                Ok(serde_dynamodb::from_hashmap(item).unwrap())
            }
            None => {
                println!("no item in database");
                Ok(Default::default())
            }
        },
        Err(error) => panic!("Error: {:?}", error),
    }
}
