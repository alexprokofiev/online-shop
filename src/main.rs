use actix_web::{cookie, get, post, web, HttpRequest, HttpResponse};
use lazy_static::lazy_static;
use std::alloc::System;
use tera::{Context, Tera};

use jsonwebtoken;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

mod model;
pub use crate::model::{CartProduct, Order, Product, User};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i64,
    exp: i64,
}

pub struct AppState {
    db: MySqlPool,
}

const SECRET_KEY: &str = "SOME SECRET KEY";
const PASSWORD_HASH_SALT: &str = "polina loh";

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
    HttpResponse::Ok().body(TEMPLATES.render("home.html", &Context::new()).unwrap())
}

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    let mut ctx = Context::new();

    let c = req.cookie("email");

    return match c {
        Some(s) => {
            ctx.insert("email", &s.to_string().strip_prefix("email="));

            HttpResponse::Ok().body(TEMPLATES.render("index.html", &ctx).unwrap())
        }
        None => HttpResponse::Ok().body(TEMPLATES.render("index.html", &ctx).unwrap()),
    };
}

#[get("/catalog")]
async fn catalog(data: web::Data<AppState>) -> HttpResponse {
    let products: Vec<Product> = sqlx::query_as!(Product, r#"SELECT * FROM products"#)
        .fetch_all(&data.db)
        .await
        .unwrap();

    let mut ctx = Context::new();

    ctx.insert("products", &products);

    HttpResponse::Ok().body(TEMPLATES.render("catalog.html", &ctx).unwrap())
}

#[get("/login")]
async fn get_login() -> HttpResponse {
    let mut ctx = Context::new();

    ctx.insert("signin", &true);

    HttpResponse::Ok().body(TEMPLATES.render("login.html", &ctx).unwrap())
}

#[get("/signup")]
async fn signup() -> HttpResponse {
    let mut ctx = Context::new();

    ctx.insert("signin", &false);

    HttpResponse::Ok().body(TEMPLATES.render("login.html", &ctx).unwrap())
}

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

#[post("/login")]
async fn post_login(data: web::Data<AppState>, form: web::Form<LoginForm>) -> HttpResponse {
    let mut hasher = Sha512::new();
    hasher.update(PASSWORD_HASH_SALT);
    hasher.update(form.password.clone());

    let users: Vec<User> = sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE email = ? AND password = ?"#,
        form.email,
        format!("{:x}", hasher.finalize())
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    if users.len() != 1 {
        return HttpResponse::BadRequest().body("Login failed");
    }

    let claims = &Claims {
        user_id: users[0].id,
        exp: i64::MAX,
    };

    let token: String = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .unwrap();

    let mut ctx = Context::new();

    ctx.insert("email", &users[0].email.clone());

    HttpResponse::Ok()
        .cookie(cookie::Cookie::build("token", token).finish())
        .cookie(cookie::Cookie::build("email", &users[0].email).finish())
        .body(TEMPLATES.render("index.html", &ctx).unwrap())
}

#[derive(Deserialize)]
struct AddToCartForm {
    product_id: i64,
}

#[post("clear_cart")]
async fn clear_cart(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let _ = sqlx::query!(
        "DELETE FROM order_products where order_id = (select id from orders where user_id = ?)",
        auth(req),
    )
        .execute(&data.db)
        .await;

    HttpResponse::Ok().body("")
}

fn auth(req: HttpRequest) -> i64 {
    return match req.cookie("token") {
        Some(s) => {
            let q = s.to_string().clone();

            let token = q.strip_prefix("token=").unwrap_or("");

            if token.len() == 0 {
                return 0;
            }

            match jsonwebtoken::decode::<Claims>(
                token,
                &jsonwebtoken::DecodingKey::from_secret(SECRET_KEY.as_bytes()),
                &jsonwebtoken::Validation::default(),
            ) {
                Ok(token_data) => {
                    token_data.claims.user_id
                },
                Err(e) => {
                    println!("{:?}", e);

                    0
                }
            }
        },
        None => 0,
    }
}

#[derive(Deserialize)]
struct ChangeQuantityForm {
    product_id: i64,
    quantity: i64,
}

#[post("/change_quantity")]
async fn change_quantity(
    req: HttpRequest,
    data: web::Data<AppState>,
    form: web::Form<ChangeQuantityForm>,
) -> HttpResponse {
    let user_id = auth(req);

    let _ = sqlx::query!(
        "UPDATE order_products SET quantity = ? WHERE order_id = (select id from orders where user_id = ?) AND product_id = ?",
        form.quantity,
        user_id,
        form.product_id
    )
    .execute(&data.db)
    .await;

    HttpResponse::Ok().body("")
}

#[post("/add_to_cart")]
async fn add_to_cart(
    req: HttpRequest,
    data: web::Data<AppState>, 
    form: web::Form<AddToCartForm>,
) -> HttpResponse {
    let user_id = auth(req);

    let mut order: Vec<Order> = sqlx::query_as!(
        Order,
        r#"SELECT
            *
        FROM
            orders
        WHERE user_id = ?"#,
        user_id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    if order.len() == 0 {
        let _ = sqlx::query!(
            "INSERT INTO orders (user_id) VALUES (?);",
            user_id,
        )
        .execute(&data.db)
        .await;

        order = sqlx::query_as!(
            Order,
            r#"SELECT
                    *
                FROM
                    orders
                WHERE user_id = ?"#,
            user_id
        )
        .fetch_all(&data.db)
        .await
        .unwrap();
    }

    let _ = sqlx::query!(
        "INSERT INTO
            `order_products` (
                `order_id`,
                `product_id`,
                `quantity`
            )
        VALUES
            (?, ?, ?);",
        order[0].id,
        form.product_id,
        1,
    )
    .execute(&data.db)
    .await;

    HttpResponse::Ok().body("")
}

#[get("/cart")]
async fn cart(req : HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let user_id = auth(req);

    let products: Vec<CartProduct> = sqlx::query_as!(
        CartProduct,
        r#"SELECT
            op.order_id,
            op.product_id,
            op.quantity,
            p.name,
            p.cost,
            p.desc,
            p.image
        FROM
            order_products op
        JOIN products p ON (p.id = op.product_id)
        JOIN orders o ON (o.id = op.order_id)
            WHERE o.user_id = ?"#,
        user_id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let mut ctx = Context::new();

    ctx.insert("products", &Vec::<CartProduct>::new());
    ctx.insert("total", &0);

    if products.len() != 0 {
        let total: f64 = products
            .iter()
            .map(|a| a.cost * a.quantity as f64)
            .reduce(|a, b| a + b)
            .unwrap();

        ctx.insert("products", &products);
        ctx.insert("total", &total);
    }

    HttpResponse::Ok().body(TEMPLATES.render("cart.html", &ctx).unwrap())
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
            .service(post_login)
            .service(cart)
            .service(add_to_cart)
            .service(clear_cart)
            .service(signup)
            .service(change_quantity)
    })
    .workers(available_parallelism().unwrap().get())
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
