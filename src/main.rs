use mongodb::{Client, options::ClientOptions};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Currency {
    code: String,
    name: String,
    symbol: String,
    status: String,
}


#[tokio::main]
async fn main() -> mongodb::error::Result<()>  {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:password@localhost:27017/rust-local?authSource=admin").await?;

    client_options.app_name = Some("Rust App".to_string());

    println!("Options: {:?}", client_options);

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    // Get a handle to a database.
    let db = client.database("cp-local");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    let collection = db.collection::<Currency>("currencies");

    // Query the books in the collection with a filter and an option.
    // let filter = doc! { "code": "USD" };
    let filter = doc! { };
    let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = collection.find(filter, find_options).await?;

    println!("List of currencies:");
    // Iterate over the results of the cursor.
    while let Some(currency) = cursor.try_next().await? {
        println!("name: {}", currency.name);
    }

    Ok(())
}
