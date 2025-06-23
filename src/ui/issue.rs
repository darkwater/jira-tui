//! Issue model and helpers for Jira TUI.

use jira_v3_openapi::models::IssueBean;
use ratatui::style::Color;

use crate::ui::theme::Theme;

#[derive(Debug, Clone, PartialEq)]
pub struct Issue {
    pub id: String,
    pub summary: String,
    pub description: String,
    pub issue_type: Option<String>,
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub story_points: Option<f64>,
    pub parent_epic: Option<String>,
    // Add more fields as needed (e.g., assignee, etc.)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
    Other(String),
}

impl Priority {
    pub const fn color(&self, theme: &Theme) -> Color {
        match self {
            Priority::High => theme.red,
            Priority::Medium => theme.yellow,
            Priority::Low => theme.blue,
            Priority::Other(_) => theme.yellow,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Review,
    Test,
    Done,
    Other(String),
}

impl Priority {
    pub fn from_jira_str(s: &str) -> Self {
        let s_lower = s.to_lowercase();
        if s_lower.starts_with("high") {
            Priority::High
        } else if s_lower.starts_with("med") {
            Priority::Medium
        } else if s_lower.starts_with("low") {
            Priority::Low
        } else {
            Priority::Other(s.to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
            Priority::Other(s) => s,
        }
    }
}

impl Status {
    pub const fn color(&self, theme: &Theme) -> Color {
        match self {
            Status::Todo => theme.white,
            Status::InProgress => theme.cyan,
            Status::Review => theme.magenta,
            Status::Test => theme.blue,
            Status::Done => theme.green,
            Status::Other(_) => theme.gray,
        }
    }

    pub fn from_jira_str(s: &str) -> Self {
        let s_lower = s.to_lowercase();
        if s_lower.contains("todo") {
            Status::Todo
        } else if s_lower.contains("progress") {
            Status::InProgress
        } else if s_lower.contains("review") {
            Status::Review
        } else if s_lower.contains("test") {
            Status::Test
        } else if s_lower.contains("done") {
            Status::Done
        } else {
            Status::Other(s.to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "In Progress",
            Status::Review => "Review",
            Status::Test => "Test",
            Status::Done => "Done",
            Status::Other(s) => s,
        }
    }
}

impl Issue {
    pub fn new<T: Into<String>>(summary: T, description: T) -> Self {
        Self {
            id: String::new(),
            summary: summary.into(),
            description: description.into(),
            issue_type: None,
            status: None,
            priority: None,
            story_points: None,
            parent_epic: None,
        }
    }

    /// Map from Jira API model to internal Issue struct.
    pub fn from_jira(jira: &IssueBean) -> Self {
        fn adf_to_plain_text(adf: &serde_json::Value) -> String {
            match adf {
                serde_json::Value::Object(map) => {
                    if let Some(content) = map.get("content") {
                        adf_to_plain_text(content)
                    } else if let Some(text) = map.get("text") {
                        text.as_str().unwrap_or("").to_string()
                    } else {
                        "".to_string()
                    }
                }
                serde_json::Value::Array(arr) => arr
                    .iter()
                    .map(adf_to_plain_text)
                    .collect::<Vec<_>>()
                    .join(""),
                _ => "".to_string(),
            }
        }

        let id = jira.key.clone().unwrap_or_else(|| "<no id>".to_string());

        let (summary, description, issue_type, status, priority, story_points, parent_epic) =
            if let Some(fields) = &jira.fields {
                let summary = fields
                    .get("summary")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "<no summary>".to_string());
                let description = match fields.get("description") {
                    Some(val) => {
                        if let Some(s) = val.as_str() {
                            s.to_string()
                        } else {
                            adf_to_plain_text(val)
                        }
                    }
                    None => "".to_string(),
                };
                let issue_type = fields
                    .get("issuetype")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let status = fields
                    .get("status")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                    .map(Status::from_jira_str);
                let priority = fields
                    .get("priority")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                    .map(Priority::from_jira_str);
                let story_points = fields.get("customfield_10016").and_then(|v| v.as_f64());
                let parent_epic = fields
                    .get("parent")
                    .and_then(|v| v.get("fields"))
                    .and_then(|v| v.get("summary"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                (summary, description, issue_type, status, priority, story_points, parent_epic)
            } else {
                ("<no summary>".to_string(), "".to_string(), None, None, None, None, None)
            };
        Self {
            id,
            summary,
            description,
            issue_type,
            status,
            priority,
            story_points,
            parent_epic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_issue() {
        let issue = Issue::new("Test", "Description");
        assert_eq!(issue.summary, "Test");
        assert_eq!(issue.description, "Description");
        assert_eq!(issue.id, "");
        assert!(issue.issue_type.is_none());
        assert!(issue.status.is_none());
        assert!(issue.priority.is_none());
        assert!(issue.story_points.is_none());
        assert!(issue.parent_epic.is_none());
    }
}
