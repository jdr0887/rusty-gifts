use chrono::prelude::*;
use diesel::*;
use serde::{Deserialize, Serialize};

use super::schema::*;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable, Insertable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
}

impl NewUser {
    pub fn new(email: String, password: String) -> NewUser {
        NewUser {
            email,
            password,
            first_name: None,
            last_name: None,
            phone: None,
        }
    }
}

impl From<shared::RegisterRequestBody> for NewUser {
    fn from(body: shared::RegisterRequestBody) -> NewUser {
        let mut user = NewUser::new(body.email, body.password);
        user.first_name = body.first_name;
        user.last_name = body.last_name;
        user.phone = body.phone;
        user
    }
}

impl From<shared::LoginRequestBody> for NewUser {
    fn from(body: shared::LoginRequestBody) -> NewUser {
        let mut user = NewUser::new(body.email, body.password);
        user
    }
}

impl From<User> for shared::MinimalUserInfo {
    fn from(body: User) -> shared::MinimalUserInfo {
        let user = shared::MinimalUserInfo {
            id: body.id,
            email: body.email,
            first_name: body.first_name,
            last_name: body.last_name,
            phone: body.phone,
        };
        user
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "destinations"]
pub struct Destination {
    pub id: i32,
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable, Insertable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "destinations"]
pub struct NewDestination {
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
}

impl NewDestination {
    pub fn new(name: String, street: String, city: String, state: String, postal_code: String) -> NewDestination {
        NewDestination {
            name,
            street,
            city,
            state,
            postal_code,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "gift_ideas"]
pub struct GiftIdea {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub url: Option<String>,
    pub date_added: NaiveDateTime,
    pub date_last_modified: NaiveDateTime,
    pub date_reserved: Option<NaiveDateTime>,
    pub owner_id: i32,
    pub recipient_user_id: i32,
    pub reserved_by_user_id: Option<i32>,
}
// pub owner_id: i32,
// pub recipient_user_id: i32,
// pub reserved_by_user_id: Option<i32>,

impl From<GiftIdea> for shared::GiftIdeaResponseBody {
    fn from(body: GiftIdea) -> shared::GiftIdeaResponseBody {
        let gift_idea = shared::GiftIdeaResponseBody {
            id: body.id,
            title: body.title,
            description: body.description,
            price: body.price,
            url: body.url,
            owner_id: body.owner_id,
            recipient_user_id: body.recipient_user_id,
            reserved_by_user_id: body.reserved_by_user_id,
        };
        gift_idea
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable, Insertable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "gift_ideas"]
pub struct NewGiftIdea {
    pub title: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub url: Option<String>,
    pub date_added: NaiveDateTime,
    pub date_last_modified: NaiveDateTime,
    pub date_reserved: Option<NaiveDateTime>,
    pub owner_id: i32,
    pub recipient_user_id: i32,
    pub reserved_by_user_id: Option<i32>,
}

impl NewGiftIdea {
    pub fn new(title: String, owner_id: i32, recipient_user_id: i32) -> NewGiftIdea {
        NewGiftIdea {
            title,
            description: None,
            price: None,
            url: None,
            date_added: Utc::now().naive_utc(),
            date_last_modified: Utc::now().naive_utc(),
            date_reserved: None,
            owner_id,
            recipient_user_id,
            reserved_by_user_id: None,
        }
    }
}

impl From<shared::GiftIdeaRequestBody> for NewGiftIdea {
    fn from(body: shared::GiftIdeaRequestBody) -> NewGiftIdea {
        let mut gift_idea = NewGiftIdea::new(body.title, body.owner_id, body.recipient_user_id);
        gift_idea.description = body.description;
        gift_idea.price = body.price;
        gift_idea.url = body.url;
        gift_idea
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[belongs_to(User)]
#[belongs_to(Destination)]
#[table_name = "user_destinations"]
pub struct UserDestination {
    pub id: i32,
    pub user_id: i32,
    pub destination_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable, Insertable, AsChangeset, Associations)]
#[table_name = "user_destinations"]
pub struct NewUserDestination {
    pub user_id: i32,
    pub destination_id: i32,
}

impl NewUserDestination {
    pub fn new(user_id: i32, destination_id: i32) -> NewUserDestination {
        NewUserDestination { user_id, destination_id }
    }
}
