use std::alloc::System;
use actix_web::{get, post, web, cookie, HttpResponse};
use tera::{Tera, Context};
use lazy_static::lazy_static;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

mod model;
pub use crate::model::User;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub struct AppState {
    db: MySqlPool,
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

#[global_allocator]
static A: System = System;

#[get("/home")]
async fn home() -> HttpResponse {
    HttpResponse::Ok().body(
        TEMPLATES.render("home.html", &Context::new()).unwrap()
    )
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(
        TEMPLATES.render("index.html", &Context::new()).unwrap()
    )
}

#[get("/catalog")]
async fn catalog() -> HttpResponse {
    HttpResponse::Ok().body(
        TEMPLATES.render("catalog.html", &Context::new()).unwrap()
    )
}

#[get("/login")]
async fn get_login() -> HttpResponse {
    HttpResponse::Ok().body(
        TEMPLATES.render("login.html", &Context::new()).unwrap()
    )
}

#[post("/login")]
async fn post_login(data: web::Data<AppState>) -> HttpResponse {
    let users: Vec<User> = sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE email = ? AND password = ?"#,
        "JGnX2@example.com",
        "123456"
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    if users.len() != 1 {
        return HttpResponse::BadRequest().body("Login failed");
    }

    let claims = &Claims{
        sub: "".to_string(),
        company: "".to_string(),
        exp: 0,
    };

    let token: String = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret")).unwrap();

    HttpResponse::Ok()
        .cookie(cookie::Cookie::build("token", token).finish())
        .body(
            TEMPLATES.render("index.html", &Context::new()).unwrap()
        )
}

#[get("/cart")]
async fn cart() -> HttpResponse {
    HttpResponse::Ok().body(
        TEMPLATES.render("catalog.html", &Context::new()).unwrap()
    )
}

#[get("/connect")]
async fn connect() -> HttpResponse {
    HttpResponse::Ok().body(
        TEMPLATES.render("connect.html", &Context::new()).unwrap()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    use actix_web::{App, HttpServer};
    use std::thread::available_parallelism;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(home)
            .service(index)
            .service(catalog)
            .service(get_login)
            .service(cart)
            .service(connect)
    }).workers(available_parallelism().unwrap().get())
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
