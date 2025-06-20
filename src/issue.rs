//! Issue model and helpers for Jira TUI.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub title: String,
    pub description: String,
    // Add more fields as needed (e.g., status, id, priority, etc.)
}

impl Issue {
    pub fn new<T: Into<String>>(title: T, description: T) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
        }
    }

    /// Map from Jira API model to internal Issue struct.
    pub fn from_jira(jira: &jira_v3_openapi::models::IssueBean) -> Self {
        let (title, description) = if let Some(fields) = &jira.fields {
            let title = fields
                .get("summary")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "<no summary>".to_string());
            let description = fields
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "".to_string());
            (title, description)
        } else {
            ("<no summary>".to_string(), "".to_string())
        };
        Self { title, description }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_issue() {
        let issue = Issue::new("Test", "Description");
        assert_eq!(issue.title, "Test");
        assert_eq!(issue.description, "Description");
    }
}
