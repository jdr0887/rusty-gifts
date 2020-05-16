use diesel::prelude::*;

use crate::models;
use crate::schema::users;
use actix_web::*;

pub fn login(user: &models::NewUser, conn: &SqliteConnection) -> Result<Option<models::User>, diesel::result::Error> {
    let user = users::table
        .filter(users::dsl::email.eq(user.email.to_string()))
        .filter(users::dsl::password.eq(user.password.to_string()))
        .first::<models::User>(conn)
        .optional()?;
    Ok(user)
}

pub fn find_all(conn: &SqliteConnection) -> Result<Vec<shared::MinimalUserInfo>, diesel::result::Error> {
    let found_users = users::table.load::<models::User>(conn).expect("Error loading users");
    let results = found_users.iter().map(|e| e.clone().into()).collect();
    Ok(results)
}

pub fn find_by_id(uid: i32, conn: &SqliteConnection) -> Result<Option<models::User>, diesel::result::Error> {
    let user = users::table.filter(users::dsl::id.eq(uid)).first::<models::User>(conn).optional()?;
    Ok(user)
}

pub fn find_by_email(mail: String, conn: &SqliteConnection) -> Result<Option<models::User>, diesel::result::Error> {
    let user = users::table.filter(users::dsl::email.eq(mail)).first::<models::User>(conn).optional()?;
    Ok(user)
}

pub fn update(user: &models::NewUser, conn: &SqliteConnection) -> Result<models::User, diesel::result::Error> {
    diesel::update(users::table.filter(users::dsl::email.eq(user.email.to_string())))
        .set(user)
        .execute(conn)
        .unwrap();
    let user = users::table.filter(users::dsl::email.eq(user.email.to_string())).first::<models::User>(conn)?;
    Ok(user)
}

pub fn add(new_user: &models::NewUser, conn: &SqliteConnection) -> Result<models::User, diesel::result::Error> {
    // RETURNING is not supported by sqlite...the following will work w/ postgresql
    //let user = diesel::insert_into(users).values(&new_user).get_result(conn).expect("Error savig");
    diesel::insert_into(users::table).values(new_user).execute(conn)?;
    let user = users::table.filter(users::dsl::email.eq(new_user.email.to_string())).first::<models::User>(conn)?;
    Ok(user)
}
