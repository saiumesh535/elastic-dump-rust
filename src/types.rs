use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct ScrollAPI {
    pub scroll: String,
    pub scroll_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Hits {
    pub hits: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Scroll {
    pub _scroll_id: Option<String>,
    pub hits: Hits,
}
