#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    message: String,
}


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

#[derive(Serialize, Deserialize, Debug)]
pub struct AddProject {
    #[serde(rename = "repositoryProvider")]
    pub repository_provider: String,
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}
