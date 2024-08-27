mod bot;
mod db;
mod models;
mod handlers;

#[tokio::main]
async fn main() {
    // Initialize and configure the bot and database
    let tasks_collection = db::get_tasks_collection().await;

    // Initialize and run the bot
    bot::run(tasks_collection).await;
}