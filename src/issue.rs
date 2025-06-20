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
