use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ShowModel {
    pub id: String,
    pub owner: String,
    pub title: String,
    pub description: String,
    pub view_code: Option<String>,
}

// models for what should be recieved in params/body of POST, PUT, GET, DELETE request hitting the /shows endpoint

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserShowsSchema {
    pub favorites: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateShowSchema {
    pub id: String,
    pub owner: String,
    pub title: String,
    pub description: String,
    pub view_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateShowSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub view_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteShowSchema {
    pub id: String,
    pub owner: String,
}

// response models for requests hitting the /shows endpoint

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ShowResponse {
    pub id: String,
    pub owner: String,
    pub title: String,
    pub description: String,
    pub viewCode: String,
}