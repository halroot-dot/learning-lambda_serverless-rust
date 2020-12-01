use lambda::{handler_fn, Context};
use lambda_runtime::error::HandlerError;
use rusoto_core::Region;

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;

use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Serialize, Deserialize, Debug, Default)]
struct CustomEvent {
    id: String,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct CustomOutput {
    id: String,
    first_name: String,
    last_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(put_handler)).await?;
    Ok(())
}

async fn put_handler(event: CustomEvent, _ctx: Context) -> Result<CustomOutput, HandlerError> {
    println!("input: CustomEvent is {:?}", event);

    let mut create_key = HashMap::new();
    create_key.insert(
        String::from("id"),
        AttributeValue {
            s: Some(event.id),
            ..Default::default()
        },
    );

    create_key.insert(
        String::from("first_name"),
        AttributeValue {
            s: Some(event.first_name),
            ..Default::default()
        },
    );

    create_key.insert(
        String::from("last_name"),
        AttributeValue {
            s: Some(event.last_name),
            ..Default::default()
        },
    );

    let input = PutItemInput {
        table_name: String::from("learning_lambda_rust"),
        item: create_key,
        ..Default::default()
    };

    let client = DynamoDbClient::new(Region::ApNortheast1);
    match client.put_item(input).await {
        Ok(result) => match result.attributes {
            Some(_) => {
                println!("some");
                Ok(Default::default())
            }
            None => {
                println!("result is {:?}", result);
                println!("none");
                Ok(Default::default())
            }
        },
        Err(error) => panic!("Error: {:?}", error),
    }
}
