use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct LoggedUser {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterRequestBody {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterResponseBody {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinimalUserInfo {
    pub id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginResponseBody {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftIdeaRequestBody {
    pub title: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub url: Option<String>,
    pub owner_id: i32,
    pub recipient_user_id: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftIdeaResponseBody {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub url: Option<String>,
    pub owner_id: i32,
    pub recipient_user_id: i32,
    pub reserved_by_user_id: Option<i32>,
}
