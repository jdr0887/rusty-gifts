use serde::{Deserialize, Serialize};

use super::schema::destinations;
use super::schema::gifts;
use super::schema::user_destinations;
use super::schema::user_gifts;
use super::schema::users;

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
    // pub username: String,
    // pub password: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

impl NewUser {
    pub fn new(email: String, password: String) -> NewUser {
        NewUser {
            email: email,
            password: password,
            first_name: None,
            last_name: None,
            phone: None,
        }
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
            name: name,
            street: street,
            city: city,
            state: state,
            postal_code: postal_code,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[table_name = "gifts"]
pub struct Gift {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub url: Option<String>,
    pub intended_for_user_id: Option<i32>,
    pub reserved_by_user_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable, Insertable, AsChangeset, Associations)]
#[table_name = "gifts"]
pub struct NewGift {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub url: Option<String>,
    pub intended_for_user_id: Option<i32>,
    pub reserved_by_user_id: Option<i32>,
}

impl NewGift {
    pub fn new(name: String, intended_for_user_id: i32, reserved_by_user_id: i32) -> NewGift {
        NewGift {
            name: name,
            description: None,
            price: None,
            url: None,
            intended_for_user_id: None,
            reserved_by_user_id: None,
        }
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
        NewUserDestination {
            user_id: user_id,
            destination_id: destination_id,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Queryable, Identifiable, Insertable, AsChangeset, Associations)]
#[belongs_to(User)]
#[belongs_to(Gift)]
#[table_name = "user_gifts"]
pub struct UserGift {
    pub id: i32,
    pub user_id: i32,
    pub gift_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable, Insertable, AsChangeset, Associations)]
#[table_name = "user_gifts"]
pub struct NewUserGift {
    pub user_id: i32,
    pub gift_id: i32,
}

impl NewUserGift {
    pub fn new(user_id: i32, gift_id: i32) -> NewUserGift {
        NewUserGift {
            user_id: user_id,
            gift_id: gift_id,
        }
    }
}
