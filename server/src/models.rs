use diesel::*;

use serde::{Deserialize, Serialize};

use super::schema::*;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
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

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
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
#[table_name = "gift_ideas"]
pub struct GiftIdea {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub url: Option<String>,
    pub intended_for_user_id: Option<i32>,
    pub reserved_by_user_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable, Insertable, AsChangeset, Associations)]
#[table_name = "gift_ideas"]
pub struct NewGiftIdea {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub url: Option<String>,
    pub intended_for_user_id: Option<i32>,
    pub reserved_by_user_id: Option<i32>,
}

impl NewGiftIdea {
    pub fn new(name: String, intended_for_user_id: i32, reserved_by_user_id: i32) -> NewGiftIdea {
        NewGiftIdea {
            name,
            description: None,
            price: None,
            url: None,
            intended_for_user_id: None,
            reserved_by_user_id: None,
        }
    }
}

impl From<shared::LoginRequestBody> for NewUser {
    fn from(body: shared::LoginRequestBody) -> NewUser {
        let mut user = NewUser::new(body.email, body.password);
        user
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

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[belongs_to(User)]
#[belongs_to(GiftIdea)]
#[table_name = "user_gift_ideas"]
pub struct UserGiftIdeas {
    pub id: i32,
    pub user_id: i32,
    pub gift_idea_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable, Insertable, AsChangeset, Associations)]
#[table_name = "user_gift_ideas"]
pub struct NewUserGiftIdeas {
    pub user_id: i32,
    pub gift_idea_id: i32,
}

impl NewUserGiftIdeas {
    pub fn new(user_id: i32, gift_idea_id: i32) -> NewUserGiftIdeas {
        NewUserGiftIdeas { user_id, gift_idea_id }
    }
}
