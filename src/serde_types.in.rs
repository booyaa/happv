#[doc(hidden)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    message: String,
}

/// Partial implementation of response return by the Projects endpoint
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "projectId")]
    pub project_id: usize,
    #[serde(rename = "accountId")]
    pub account_id: usize,
    pub slug: String,
    #[serde(rename = "accountName")]
    pub account_name: String,
}

#[doc(hidden)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AddProject {
    #[serde(rename = "repositoryProvider")]
    pub repository_provider: String,
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}
