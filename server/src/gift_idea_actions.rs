use crate::diesel::sqlite::Sqlite;
use diesel::debug_query;
use diesel::prelude::*;

use crate::models;
use crate::schema::gift_ideas;
use actix_web::*;

pub fn find_all(conn: &SqliteConnection) -> Result<Vec<models::GiftIdea>, diesel::result::Error> {
    let results = gift_ideas::table.load::<models::GiftIdea>(conn).expect("failed to find all");
    Ok(results)
}

// pub fn find_my_gifts(uid: i32, conn: &SqliteConnection) -> Result<Vec<models::GiftIdea>, diesel::result::Error> {
//     let results = gift_ideas::table
//         .filter(gift_ideas::dsl::recipient_user_id.eq(uid))
//         .load::<models::GiftIdea>(conn)
//         .expect("failed to find all");
//     Ok(results)
// }

pub fn find_by_id(gid: i32, conn: &SqliteConnection) -> Result<Option<models::GiftIdea>, diesel::result::Error> {
    let gift = gift_ideas::table.filter(gift_ideas::dsl::id.eq(gid));
    println!("{}", debug_query::<Sqlite, _>(&gift).to_string());
    let results = gift.first::<models::GiftIdea>(conn).optional()?;
    Ok(results)
}

pub fn add(new_gift: &models::NewGiftIdea, conn: &SqliteConnection) -> Result<models::GiftIdea, diesel::result::Error> {
    // RETURNING is not supported by sqlite...the following will work w/ postgresql
    //let user = diesel::insert_into(users).values(&new_user).get_result(conn).expect("Error savig");
    let insert = diesel::insert_into(gift_ideas::table).values(new_gift);
    println!("{}", debug_query::<Sqlite, _>(&insert).to_string());
    insert.execute(conn)?;
    let gift = gift_ideas::table
        .filter(gift_ideas::dsl::title.eq(new_gift.title.to_string()))
        .first::<models::GiftIdea>(conn)?;
    Ok(gift)
}

pub fn reserve(gid: i32, uid: i32, conn: &SqliteConnection) -> Result<shared::GiftIdeaResponseBody, diesel::result::Error> {
    let mut gift = gift_ideas::table.filter(gift_ideas::dsl::id.eq(gid)).first::<models::GiftIdea>(conn).optional()?.unwrap();
    gift.reserved_by_user_id = Some(uid);
    let updated_row = diesel::update(gift_ideas::table.find(gid)).set(gift).execute(conn)?;
    let gift = gift_ideas::table.filter(gift_ideas::dsl::id.eq(gid)).first::<models::GiftIdea>(conn)?;
    Ok(gift.into())
}

pub fn unreserve(gid: i32, conn: &SqliteConnection) -> Result<shared::GiftIdeaResponseBody, diesel::result::Error> {
    let mut gift = gift_ideas::table.filter(gift_ideas::dsl::id.eq(gid)).first::<models::GiftIdea>(conn).optional()?.unwrap();
    gift.reserved_by_user_id = None;
    let updated_row = diesel::update(gift_ideas::table.find(gid)).set(gift).execute(conn)?;
    let gift = gift_ideas::table.filter(gift_ideas::dsl::id.eq(gid)).first::<models::GiftIdea>(conn)?;
    Ok(gift.into())
}

pub fn delete(gid: i32, conn: &SqliteConnection) -> Result<bool, diesel::result::Error> {
    let num_deleted = diesel::delete(gift_ideas::table.filter(gift_ideas::dsl::id.eq(gid))).execute(conn)?;
    println!("num_deleted: {}", num_deleted);
    Ok(num_deleted == 1)
}

pub fn update(gift: &models::GiftIdea, conn: &SqliteConnection) -> Result<models::GiftIdea, diesel::result::Error> {
    diesel::update(gift_ideas::table.filter(gift_ideas::dsl::id.eq(gift.id))).set(gift).execute(conn).unwrap();
    let gift = gift_ideas::table.filter(gift_ideas::dsl::id.eq(gift.id)).first::<models::GiftIdea>(conn)?;
    Ok(gift)
}
