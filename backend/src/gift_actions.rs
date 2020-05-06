use diesel::prelude::*;

use crate::models;
use crate::schema::gifts::dsl::*;
use actix_web::*;

pub fn find_by_id(gid: i32, conn: &SqliteConnection) -> Result<Option<models::Gift>, diesel::result::Error> {
    let gift = gifts.filter(id.eq(gid)).first::<models::Gift>(conn).optional()?;
    Ok(gift)
}

pub fn add(new_gift: &models::NewGift, conn: &SqliteConnection) -> Result<models::Gift, diesel::result::Error> {
    // RETURNING is not supported by sqlite...the following will work w/ postgresql
    //let user = diesel::insert_into(users).values(&new_user).get_result(conn).expect("Error savig");
    diesel::insert_into(gifts).values(new_gift).execute(conn)?;
    let gift = gifts.filter(name.eq(new_gift.name.to_string())).first::<models::Gift>(conn)?;
    Ok(gift)
}

pub fn update(gift: &models::Gift, conn: &SqliteConnection) -> Result<models::Gift, diesel::result::Error> {
    diesel::update(gifts.filter(id.eq(gift.id))).set(gift).execute(conn).unwrap();
    let gift = gifts.filter(id.eq(gift.id)).first::<models::Gift>(conn)?;
    Ok(gift)
}
