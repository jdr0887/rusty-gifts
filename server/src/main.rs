#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix_files::{Files, NamedFile};
use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod gift_actions;
mod models;
mod schema;
mod user_actions;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[post("users/add")]
async fn add_user(pool: web::Data<DbPool>, form: web::Json<shared::RegisterRequestBody>) -> Result<HttpResponse, Error> {
    let new_user = form.into_inner().into();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || user_actions::add(&new_user, &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("users/login")]
async fn login(pool: web::Data<DbPool>, form: web::Json<shared::LoginRequestBody>) -> Result<HttpResponse, Error> {
    let new_user = form.into_inner().into();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || user_actions::login(&new_user, &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("users/update")]
async fn update_user(pool: web::Data<DbPool>, form: web::Json<models::NewUser>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || user_actions::update(&form.into_inner(), &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("users/find_all")]
async fn find_all_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let users = web::block(move || user_actions::find_all(&conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if !users.is_empty() {
        Ok(HttpResponse::Ok().json(users))
    } else {
        let res = HttpResponse::NotFound().body("No users in database");
        Ok(res)
    }
}

#[get("users/find_by_id/{user_id}")]
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

#[get("users/find_by_email/{user_email}")]
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

#[post("gifts/add")]
async fn add_gift(pool: web::Data<DbPool>, form: web::Json<models::NewGiftIdea>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let gift = web::block(move || gift_actions::add(&form.into_inner(), &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(gift))
}

#[post("/gifts/update")]
async fn update_gift(pool: web::Data<DbPool>, form: web::Json<models::GiftIdea>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let gift = web::block(move || gift_actions::update(&form.into_inner(), &conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(gift))
}

#[get("gifts/find_by_id/{gift_id}")]
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

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./client/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let bind = "127.0.0.1:8000";
    //let bind = "192.168.0.9:8000";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v1/")
                    .service(add_user)
                    .service(login)
                    .service(update_user)
                    .service(find_user_by_id)
                    .service(find_user_by_email)
                    .service(find_all_users)
                    .service(add_gift)
                    .service(update_gift)
                    .service(find_gift_by_id)
                    .default_service(web::route().to(web::HttpResponse::NotFound)),
            )
            .service(Files::new("/pkg", "./client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind(&bind)?
    .run()
    .await
}
