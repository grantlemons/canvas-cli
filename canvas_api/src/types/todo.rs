use super::Assignment;
use serde::Deserialize;

// GET /api/v1/users/self/todo
#[derive(Debug, Deserialize)]
pub struct Todo {
    #[serde(alias = "type")]
    pub todo_type: TodoType,
    pub assignment: Assignment,
    #[serde(alias = "ignore")]
    pub ignore_url: String,
    #[serde(alias = "ignore_permanently")]
    pub ignore_permanently_url: String,
    pub html_url: String,
    pub context_type: TodoContextType,
    pub group_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub enum TodoType {
    Submitting,
    Grading,
}

#[derive(Debug, Deserialize)]
pub enum TodoContextType {
    Course,
    Group,
}