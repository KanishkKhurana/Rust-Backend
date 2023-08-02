use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateEntryData{
    pub title: String,
    pub description: String, 
    pub date: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData{
    // pub id: i32,
    pub title: String,
    pub description: String, 
    pub date: i64,
}