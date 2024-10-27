use mongodb::Collection;
use crate::{handlers::task_handler, models::task::Task};
use teloxide::prelude::*;
use futures::{stream::TryStreamExt, StreamExt};

struct Bot {
    bot: teloxide::Bot,
    chat_id: ChatId,
}

impl Bot {
    fn new(chat_id: ChatId) -> Self {
        let bot = teloxide::Bot::from_env();
        Bot { bot, chat_id }
    }
}

pub async fn run(tasks_collection: Collection<Task>) {
    let bot = Bot::from_env();
    println!("{:?}", bot.token());
    let chat_id = ChatId(5601336851);

    let tasks = tasks_collection.find(None, None).await.unwrap();

    // iterate over tasks

    // tasks_collection.find(None, None).await.unwrap()
    //     .try_for_each(|task| async move {
    //         send_task_to_bot(task).await
    //     })
    //     .await?;

    match bot.send_message(chat_id, "Hello, I'm the task manager bot!").send().await {
        Ok(_) => println!("Message sent successfully"),
        Err(e) => eprintln!("Error sending message: {:?}", e),
    }
}