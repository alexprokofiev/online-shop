use actix_files::NamedFile;
use std::path::PathBuf;
use std::alloc::System;
use actix_web::{Result, HttpRequest, get};
use std::thread::available_parallelism;

#[global_allocator]
static A: System = System;

#[get("/home")]
async fn home() -> Result<NamedFile> {
    Ok(NamedFile::open("templates/home.html")?)
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("templates/index.html")?)
}

#[get("/catalog")]
async fn catalog() -> Result<NamedFile> {
    Ok(NamedFile::open("templates/catalog.html")?)
}

#[get("/login")]
async fn get_login() -> Result<NamedFile> {
    Ok(NamedFile::open("templates/login.html")?)
}

#[get("/cart")]
async fn cart() -> Result<NamedFile> {
    Ok(NamedFile::open("templates/cart.html")?)
}

#[get("/connect")]
async fn connect() -> Result<NamedFile> {
    Ok(NamedFile::open("templates/connect.html")?)
}

#[get("/main")]
async fn main_handler() -> Result<NamedFile> {
    Ok(NamedFile::open("templates/main.html")?)
}

async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let mut path: PathBuf = PathBuf::new();
    path.push("static");
    path.push(req.match_info().query("filename").parse::<String>().unwrap());

    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, web, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/static/{filename:.*}", web::get().to(static_files))
            .service(home)
            .service(index)
            .service(catalog)
            .service(get_login)
            .service(cart)
            .service(connect)
            .service(main_handler)
    }).workers(available_parallelism().unwrap().get())
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
