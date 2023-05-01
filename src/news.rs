use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub status: String,
    pub total_results: i64,
    pub articles: Vec<Article>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub source: Source,
    pub author: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub url_to_image: Option<String>,
    pub published_at: Option<String>,
    pub content: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub id: String,
    pub name: Option<String>,
}
