// #[allow(unused_imports)]
// use serde::{de, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    message: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "projectId")]
    project_id: usize,
    #[serde(rename = "accountId")]
    account_id: usize,
    // builds : Vec<Build>,
    slug: String,
    #[serde(rename = "accountName")]
    account_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddProject {
    #[serde(rename = "repositoryProvider")]
    repository_provider: String,
    #[serde(rename = "repositoryName")]
    repository_name: String,
}
