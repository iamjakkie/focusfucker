use chrono::Date;
use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, DateTime};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    Cyclic,
    LongTerm,
    OnceOff,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Snoozed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntry {
    pub date: DateTime,
    pub duration: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub description: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub created_at: DateTime,
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
    pub total_time_spent: u64,  // Total time spent in seconds
    pub time_entries: Vec<TimeEntry>,  // Track individual time entries
    pub confirmations: u32, // Number of times the bot confirmed task progress
    pub last_snooze: Option<DateTime>, // Track the last time the task was snoozed
    pub subtasks: Option<Vec<Task>>,
    pub dependent_on: Option<Vec<ObjectId>>, // Track the task that this task is dependent on
}

impl Task {
    // Utility method to create a new task with an optional list of subtasks
    pub fn new(description: String, task_type: TaskType, subtasks: Option<Vec<Task>>, dependent_on: Option<Vec<ObjectId>>) -> Self {
        Task {
            id: ObjectId::new(),
            description,
            task_type,
            status: TaskStatus::Pending,
            created_at: DateTime::now(),
            start_time: None,
            end_time: None,
            total_time_spent: 0,
            time_entries: Vec::new(),
            confirmations: 0,
            last_snooze: None,
            subtasks,
            dependent_on,
        }
    }

    // Example method to calculate the total time spent including all subtasks
    pub fn calculate_total_time_spent(&self) -> u64 {
        let mut total_time = self.total_time_spent;
        if let Some(subtasks) = &self.subtasks {
            for subtask in subtasks {
                total_time += subtask.calculate_total_time_spent();
            }
        }
        total_time
    }

    pub fn to_string(&self) -> String {
        format!("Task: {}\nType: {:?}\nStatus: {:?}\nCreated At: {}\nStart Time: {:?}\nEnd Time: {:?}\nTotal Time Spent: {}\nTime Entries: {:?}\nConfirmations: {}\nLast Snooze: {:?}\nSubtasks: {:?}\nDependent On: {:?}",
                self.description, self.task_type, self.status, self.created_at, self.start_time, self.end_time, self.total_time_spent, self.time_entries, self.confirmations, self.last_snooze, self.subtasks, self.dependent_on)
    }
}