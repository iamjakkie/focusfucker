use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskType {
    Cyclic,
    LongTerm,
    OnceOff,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Snoozed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeEntry {
    pub date: DateTime<Utc>,
    pub duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub total_time_spent: u64,
    pub time_entries: Vec<TimeEntry>,
    pub confirmations: u32,
    pub last_snooze: Option<DateTime<Utc>>,
}