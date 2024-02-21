use super::SubmissionType;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Assignment {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub due_at: DateTime<Local>,
    pub lock_at: DateTime<Local>,
    pub unlock_at: DateTime<Local>,
    pub has_overrides: bool,
    pub all_dates: Option<()>,
    pub course_id: u64,
    pub html_url: String,
    pub submissions_download_url: String,
    pub assignment_group_id: u64,
    pub allowed_extensions: Vec<String>,
    pub max_name_length: u64,
    // pub turnitin_enabled: bool,
    // pub vericite_enabled: bool,
    // pub turnitin_settings: Option<()>,
    pub grade_group_students_individually: bool,
    // pub external_tool_tag_attributes: Option<()>,
    pub peer_reviews: bool,
    pub automatic_peer_reviews: bool,
    pub peer_review_count: u64,
    pub peer_reviews_assign_at: DateTime<Local>,
    pub intra_group_peer_reviews: bool,
    pub group_category_id: u64,
    pub position: u64,
    // pub post_to_sis: bool,
    pub points_possible: f32,
    pub submission_types: Vec<SubmissionType>,
    pub has_submitted_submissions: bool,
    pub grading_type: GradingType,
    // pub only_visible_to_overrides: bool,
    pub locked_for_user: bool,
    pub lock_info: Option<String>,
    pub lock_explanation: Option<String>,
    pub discussion_topic: Option<String>,
    // pub overrides: (),
    pub omit_from_final_grade: Option<bool>,
    // pub grader_count: u64,
    pub final_grader_id: u64,
    pub allowed_attemps: i64,
    pub post_manually: bool,
    pub score_statistics: (),
    pub can_submit: Option<bool>,
    // pub annotatable_attachment_id: Option<u64>,
    pub require_lockdown_browser: Option<bool>,
    // pub important_dates: Option<bool>,
    pub graded_submissions_exist: bool,
    pub is_quiz_assignment: bool,
}

#[derive(Debug, Deserialize)]
pub enum GradingType {
    #[serde(alias = "pass_fail")]
    PassFail,
    #[serde(alias = "percent")]
    Percent,
    #[serde(alias = "letter_grade")]
    LetterGrade,
    #[serde(alias = "gpa_scale")]
    GpaScale,
    #[serde(alias = "points")]
    Points,
}