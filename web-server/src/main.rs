use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use serde::Deserialize;

pub mod db;
pub mod models;
pub mod schema;

#[derive(Deserialize)]
struct CreateMessage {
    message: String,
}

#[derive(Deserialize, Debug)]
struct CreateUser {
    name: String,
    dont_use: Option<String>,
}

#[derive(Deserialize)]
struct Login {
    name: String,
}

// #[get("/message/{id}")]
// async fn get_message(path: web::Path<String>) -> impl Responder {
//     let id = path.into_inner();
//     HttpResponse::Ok().body(format!("{}", id))
// }

// #[post("/message")]
// async fn create_message(data: web::Json<CreateMessage>) -> impl Responder {
//     HttpResponse::Ok().body(format!("{}", data.message))
// }

#[post("/create")]
async fn create_user(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        let new_user = models::InsertUser { name: &data.name };
        db::create_user(&mut conn, new_user)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/login")]
async fn login(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    data: web::Query<Login>,
) -> Result<HttpResponse, Error> {
    let data = data.into_inner();
    let name = data.name.clone();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::get_user(&mut conn, name.as_str())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().body(format!("User \"{}\" not found", data.name.clone())))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setu DB Pool
    let conn_manager = db::create_connection_pool();

    // Start Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn_manager.clone()))
            // .service(get_message)
            // .service(create_message)
            .service(login)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
