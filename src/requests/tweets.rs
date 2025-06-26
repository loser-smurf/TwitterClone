use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweetRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRetweetRequest {
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct TweetsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    20
}
