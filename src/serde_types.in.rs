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
}
