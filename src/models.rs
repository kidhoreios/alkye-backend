use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub category: String,
    pub short_description: String,
    pub content: String,
    pub image_url: Option<String>,
    pub created_at: String,
}

/// Shape of entries in `seed.json`.
#[derive(Debug, Deserialize)]
pub struct NewArticle {
    pub title: String,
    pub category: String,
    pub short_description: String,
    pub content: String,
    pub image_url: Option<String>,
    pub created_at: String,
}
