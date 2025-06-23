//! Issue model and helpers for Jira TUI.

#[derive(Debug, Clone, PartialEq)]
pub struct Issue {
    pub id: String,
    pub summary: String,
    pub description: String,
    pub issue_type: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub story_points: Option<f64>,
    pub parent_epic: Option<String>,
    // Add more fields as needed (e.g., assignee, etc.)
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
    pub fn from_jira(jira: &jira_v3_openapi::models::IssueBean) -> Self {
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
                    .map(|s| s.to_string());
                let priority = fields
                    .get("priority")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
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
