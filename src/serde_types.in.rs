use serde::{de, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    message: String,
}
