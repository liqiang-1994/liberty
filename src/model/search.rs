use crate::model::response::Page;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeySearch {
    pub key: String,
    pub search_type: String,
    pub page: Page,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoemResp {
    pub id: String,
    pub author: String,
    pub author_id: Option<String>,
    pub style: Option<String>,
    pub content: Option<String>,
    pub rhythmic: String,
    pub section: Option<String>,
    pub notes: Option<String>,
    pub strains: Option<String>,
}
