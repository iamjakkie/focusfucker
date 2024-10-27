use mongodb::Collection;
use crate::models::task::Task;
use teloxide::prelude::*;

pub async fn run(tasks_collection: Collection<Task>) {
    let bot = Bot::from_env();
    println!("{:?}", bot.token());
    let chat_id = ChatId(5601336851);

    match bot.send_message(chat_id, "Hello, I'm the task manager bot!").send().await {
        Ok(_) => println!("Message sent successfully"),
        Err(e) => eprintln!("Error sending message: {:?}", e),
    }
}