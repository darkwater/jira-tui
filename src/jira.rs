use jira_v3_openapi::apis::Error as JiraApiError;
use jira_v3_openapi::apis::configuration::Configuration;
use jira_v3_openapi::apis::issue_search_api::search_for_issues_using_jql;
use jira_v3_openapi::models::search_results::SearchResults;
use std::env;

pub struct JiraConfig {
    pub base_url: String,
    pub username: String,
    pub api_token: String,
}

impl JiraConfig {
    /// Load config from environment variables.
    /// - JIRA_TUI_URL: Base URL (e.g. https://your-domain.atlassian.net)
    /// - JIRA_TUI_USER: Username/email
    /// - JIRA_TUI_TOKEN: API token
    pub fn from_env() -> Result<Self, String> {
        let base_url = env::var("JIRA_TUI_URL").map_err(|_| "JIRA_TUI_URL not set")?;
        let username = env::var("JIRA_TUI_USER").map_err(|_| "JIRA_TUI_USER not set")?;
        let api_token = env::var("JIRA_TUI_TOKEN").map_err(|_| "JIRA_TUI_TOKEN not set")?;
        Ok(Self {
            base_url,
            username,
            api_token,
        })
    }

    pub fn to_api_config(&self) -> Configuration {
        let mut config = Configuration::new();
        config.base_path = self.base_url.clone();
        config.basic_auth = Some((self.username.clone(), Some(self.api_token.clone())));
        config
    }
}

/// Fetch issues assigned to the current user using JQL.
/// Returns the raw SearchResults from the Jira API.
pub async fn fetch_assigned_issues(
    config: &JiraConfig,
    max_results: i32,
) -> Result<
    SearchResults,
    JiraApiError<jira_v3_openapi::apis::issue_search_api::SearchForIssuesUsingJqlError>,
> {
    let api_config = config.to_api_config();
    // JQL for issues assigned to the current user, unresolved, ordered by update time.
    let jql = "assignee = currentUser() AND resolution = Unresolved ORDER BY updated DESC";
    search_for_issues_using_jql(
        &api_config,
        Some(jql),
        Some(0),
        Some(max_results),
        None, // validate_query
        None, // fields (None = all navigable)
        None, // expand
        None, // properties
        None, // fields_by_keys
        None, // jql_context
    )
    .await
}
