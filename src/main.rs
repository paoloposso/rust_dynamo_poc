use std::collections::{HashMap, hash_map::RandomState};

use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, AttributeValueUpdate, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput, UpdateItemInput};

#[tokio::main]
async fn main() { 
    let region = Region::Custom {
        endpoint: "http://localhost:8000".to_owned(),
        name: "global".to_owned(),
    };

    let client = DynamoDbClient::new(region);

    list_tables(&client).await;
    insert_user(&client, "999999".to_owned(), "Joseph".to_owned(), "Sao Paulo".to_owned()).await;
    insert_user(&client, "123456".to_owned(), "Jane".to_owned(), "Rio de Janeiro".to_owned()).await;
    insert_user(&client, "111111".to_owned(), "Paolo".to_owned(), "Sao Paulo".to_owned()).await;
    list_users_by_document(&client, "123465".to_owned()).await;
    update_user(&client, "999999".to_owned(), "Joseph Jones".to_owned(), "Rio Branco".to_owned()).await;
    delete_user(&client, "123456".to_owned()).await;
}

async fn list_tables(client: &DynamoDbClient) {
    if let Ok(output) = client.list_tables(Default::default()).await {
        if output.table_names.is_some() {
            println!("Tables list:");
            for table_name in output.table_names.unwrap() {
                println!("{}", table_name);
            }
        }
    }
}

async fn list_users_by_document(client: &DynamoDbClient, document: String) {
    let mut map: HashMap<String, AttributeValue> = HashMap::new();
     map.insert("document".to_owned(), 
    AttributeValue {
        s: Some(document.to_owned()),
        b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, ss: None
     });

    match client.get_item(
        GetItemInput { 
            table_name: "Users".to_owned(), key: map,
            attributes_to_get: None, consistent_read: None, expression_attribute_names: None, projection_expression: None, return_consumed_capacity: None }).await {
        Ok(output) => {
            println!("{:?}", output.item.to_owned());
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

async fn insert_user(client: &DynamoDbClient, document: String, name: String, city: String) {
    let mut map: HashMap<String, AttributeValue> = HashMap::new();
    map.insert("document".to_owned(), 
    AttributeValue {
        s: Some(document.to_owned()),
        b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, ss: None
    });
    map.insert("name".to_owned(), 
    AttributeValue {
        s: Some(name.to_owned()),
        b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, ss: None
    });
    map.insert("city".to_owned(), 
    AttributeValue {
        s: Some(city.to_owned()),
        b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, ss: None
    });

    match client.put_item(
        PutItemInput { 
            item: map,
            table_name: "Users".to_owned(),
            condition_expression: None, conditional_operator: None, expected: None, expression_attribute_names: None, 
            expression_attribute_values: None, return_consumed_capacity: None, return_item_collection_metrics: None, return_values: None}).await {
        Ok(output) => {
            println!("{:?}", output.to_owned());
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

async fn delete_user(client: &DynamoDbClient, document: String) {
    let mut map: HashMap<String, AttributeValue, RandomState> = HashMap::new();
    
    map.insert("document".to_owned(), AttributeValue {
        s: Some(document.to_owned()),
        b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, ss: None
    });

    match client.delete_item(
        DeleteItemInput {
            key: map,
            table_name: "Users".to_owned(),
            condition_expression: None, conditional_operator: None, expected: None, expression_attribute_names: None, 
            expression_attribute_values: None, return_consumed_capacity: None, return_item_collection_metrics: None, return_values: None}).await {
        Ok(output) => {
            println!("{:?}", output.to_owned());
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

async fn update_user(client: &DynamoDbClient, document: String, name: String, city: String) {
    let mut key_map: HashMap<String, AttributeValue, RandomState> = HashMap::new();
    key_map.insert("document".to_owned(), AttributeValue {
        s: Some(document.to_owned()),
        b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, ss: None
    });

    let mut attr_map: HashMap<String, AttributeValueUpdate, RandomState> = HashMap::new();
    attr_map.insert("name".to_owned(), 
    AttributeValueUpdate { 
        action: None,  
        value: Some(AttributeValue {
            s: Some(name.to_owned()),
            b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, ss: None
        }),
    });
    attr_map.insert("city".to_owned(), 
    AttributeValueUpdate { 
        action: None,  
        value: Some(AttributeValue {
            s: Some(city.to_owned()),
            b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, ss: None
        }),
    });

    match client.update_item(
        UpdateItemInput {
            key: key_map,
            attribute_updates: Some(attr_map),
            update_expression: None,
            table_name: "Users".to_owned(),
            condition_expression: None, conditional_operator: None, expected: None, expression_attribute_names: None, 
            expression_attribute_values: None, return_consumed_capacity: None, return_item_collection_metrics: None, return_values: None}).await {
        Ok(output) => {
            println!("{:?}", output.to_owned());
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}