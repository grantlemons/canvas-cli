use crate::types::Discussion;
use reqwest::{Client, Result};

pub async fn list_course_discussions(client: Client, course_id: u64) -> Result<Vec<Discussion>> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/discussion_topics"),
        None,
    )
    .await
}