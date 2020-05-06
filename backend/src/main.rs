#[macro_use]
extern crate diesel;

use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod gift_actions;
mod models;
mod schema;
mod user_actions;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[post("/v1/users/add")]
async fn add_user(pool: web::Data<DbPool>, form: web::Json<models::NewUser>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || user_actions::add(&form.email, &form.password, &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/v1/users/update")]
async fn update_user(pool: web::Data<DbPool>, form: web::Json<models::NewUser>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || user_actions::update(&form.into_inner(), &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/v1/users/find_by_id/{user_id}")]
async fn find_user_by_id(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user_uid = user_id.into_inner();
    let user = web::block(move || user_actions::find_by_id(user_uid, &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

#[get("/v1/users/find_by_email/{user_email}")]
async fn find_user_by_email(pool: web::Data<DbPool>, user_email: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let email = user_email.to_owned();
    let user = web::block(move || user_actions::find_by_email(email, &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with email: {}", user_email.to_owned()));
        Ok(res)
    }
}

#[post("/v1/gifts/add")]
async fn add_gift(pool: web::Data<DbPool>, form: web::Json<models::NewGift>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let gift = web::block(move || gift_actions::add(&form.into_inner(), &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(gift))
}

#[post("/v1/gifts/update")]
async fn update_gift(pool: web::Data<DbPool>, form: web::Json<models::Gift>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let gift = web::block(move || gift_actions::update(&form.into_inner(), &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(gift))
}

#[get("/v1/gifts/find_by_id/{gift_id}")]
async fn find_gift_by_id(pool: web::Data<DbPool>, gift_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let g_id = gift_id.into_inner();
    let gift = web::block(move || gift_actions::find_by_id(g_id, &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(gift) = gift {
        Ok(HttpResponse::Ok().json(gift))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", g_id));
        Ok(res)
    }
}

// #[delete("/v1/gifts/delete/{gift_id}")]
// async fn delete_gift(pool: web::Data<DbPool>, form: web::Json<models::NewGift>) -> Result<HttpResponse, Error> {
//     let conn = pool.get().expect("couldn't get db connection from pool");
//     let user = web::block(move || actions::delete_gift(&form.into_inner(), &conn)).await.map_err(|e| {
//         eprintln!("{}", e);
//         HttpResponse::InternalServerError().finish()
//     })?;
//     Ok(HttpResponse::Ok().json(user)
// }

// #[put("/v1/gifts/un_reserve/{gift_id}")]
// async fn update_gift(pool: web::Data<DbPool>, form: web::Json<models::NewGift>) -> Result<HttpResponse, Error> {
//     let conn = pool.get().expect("couldn't get db connection from pool");
//     let user = web::block(move || actions::un_reserve_gift(&form.into_inner(), &conn)).await.map_err(|e| {
//         eprintln!("{}", e);
//         HttpResponse::InternalServerError().finish()
//     })?;
//     Ok(HttpResponse::Ok().json(user))
// }

// #[get("/v1/gifts/find_my_gifts/{user_id}")]
// async fn find_gifts_by_user_id(pool: web::Data<DbPool>, gift_id: web::Path<i32>) -> Result<HttpResponse, Error> {
//     let conn = pool.get().expect("couldn't get db connection from pool");
//     let id = gift_id.into_inner();
//     let gift = web::block(move || gift_actions::find_by_user_id(id, &conn)).await.map_err(|e| {
//         eprintln!("{}", e);
//         HttpResponse::InternalServerError().finish()
//     })?;
//
//     if let Some(gift) = gift {
//         Ok(HttpResponse::Ok().json(gift))
//     } else {
//         let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", id));
//         Ok(res)
//     }
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(add_user)
            .service(update_user)
            .service(find_user_by_id)
            .service(find_user_by_email)
            .service(add_gift)
            .service(update_gift)
            .service(find_gift_by_id)
    })
    .bind(&bind)?
    .run()
    .await
}
