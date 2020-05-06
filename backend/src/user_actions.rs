use diesel::prelude::*;

use crate::models;
use crate::schema::users::dsl::*;
use actix_web::*;

pub fn find_by_id(uid: i32, conn: &SqliteConnection) -> Result<Option<models::User>, diesel::result::Error> {
    let user = users.filter(id.eq(uid)).first::<models::User>(conn).optional()?;
    Ok(user)
}

pub fn find_by_email(mail: String, conn: &SqliteConnection) -> Result<Option<models::User>, diesel::result::Error> {
    let user = users.filter(email.eq(mail)).first::<models::User>(conn).optional()?;
    Ok(user)
}

pub fn update(user: &models::NewUser, conn: &SqliteConnection) -> Result<models::User, diesel::result::Error> {
    diesel::update(users.filter(email.eq(user.email.to_string()))).set(user).execute(conn).unwrap();
    let user = users.filter(email.eq(user.email.to_string())).first::<models::User>(conn)?;
    Ok(user)
}

pub fn add(mail: &str, pwd: &str, conn: &SqliteConnection) -> Result<models::User, diesel::result::Error> {
    let new_user = models::NewUser::new(mail.to_owned(), pwd.to_owned());
    // RETURNING is not supported by sqlite...the following will work w/ postgresql
    //let user = diesel::insert_into(users).values(&new_user).get_result(conn).expect("Error savig");
    diesel::insert_into(users).values(&new_user).execute(conn)?;
    let user = users.filter(email.eq(mail.to_string())).first::<models::User>(conn)?;
    Ok(user)
}
