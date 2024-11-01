use mongodb::Collection;
use crate::{handlers::task_handler, models::task::Task};
use teloxide::prelude::*;
use futures::{stream::TryStreamExt, FutureExt, StreamExt};

const chat_id: ChatId = ChatId(5601336851);

struct Bot {
    bot: teloxide::Bot,
    chat_id: ChatId,
}

impl Bot {
    fn new() -> Self {
        let bot = teloxide::Bot::new(std::env::var("TELEGRAM_BOT_TOKEN").unwrap());
        Bot { bot, chat_id }
    }
}

pub async fn run(tasks_collection: Collection<Task>) {
    let bot = Bot::new();

    let tasks = tasks_collection.find(None, None).await.unwrap();

    // iterate over tasks

    let v: Vec<_> = tasks.collect().await;

    for task in v {
        let msg = task.unwrap();
        bot.bot.send_message(chat_id, msg.to_string()).send().await;

    }


    match bot.bot.send_message(chat_id, "Hello, I'm the task manager bot!").send().await {
        Ok(_) => println!("Message sent successfully"),
        Err(e) => eprintln!("Error sending message: {:?}", e),
    }
}