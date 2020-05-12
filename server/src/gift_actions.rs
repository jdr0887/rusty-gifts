use diesel::prelude::*;

use crate::models;
use crate::schema::gift_ideas::dsl::*;
use actix_web::*;

pub fn find_by_id(gid: i32, conn: &SqliteConnection) -> Result<Option<models::GiftIdea>, diesel::result::Error> {
    let gift = gift_ideas.filter(id.eq(gid)).first::<models::GiftIdea>(conn).optional()?;
    Ok(gift)
}

pub fn add(new_gift: &models::NewGiftIdea, conn: &SqliteConnection) -> Result<models::GiftIdea, diesel::result::Error> {
    // RETURNING is not supported by sqlite...the following will work w/ postgresql
    //let user = diesel::insert_into(users).values(&new_user).get_result(conn).expect("Error savig");
    diesel::insert_into(gift_ideas).values(new_gift).execute(conn)?;
    let gift = gift_ideas.filter(name.eq(new_gift.name.to_string())).first::<models::GiftIdea>(conn)?;
    Ok(gift)
}

pub fn update(gift: &models::GiftIdea, conn: &SqliteConnection) -> Result<models::GiftIdea, diesel::result::Error> {
    diesel::update(gift_ideas.filter(id.eq(gift.id))).set(gift).execute(conn).unwrap();
    let gift = gift_ideas.filter(id.eq(gift.id)).first::<models::GiftIdea>(conn)?;
    Ok(gift)
}
