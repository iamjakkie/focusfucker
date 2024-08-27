use mongodb::{options::ClientOptions, Client, Collection};
use std::env;
use crate::models::task::Task;

pub async fn get_tasks_collection() -> Collection<Task> {
    dotenv::dotenv().ok();

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(&mongodb_uri).await.expect("Failed to parse client options");
    let client = Client::with_options(client_options).expect("Failed to initialize client");

    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let collection_name = env::var("COLLECTION_NAME").expect("COLLECTION_NAME must be set");

    client.database(&db_name).collection::<Task>(&collection_name)
}