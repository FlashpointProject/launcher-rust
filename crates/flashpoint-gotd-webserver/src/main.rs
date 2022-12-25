use actix_files as fs;
use actix_web::{App, HttpServer};
use colored::Colorize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let addr = "127.0.0.1";
  let port = 8080;
  let link = format!("http://{}:{}", addr, port).blue();
  println!("Starting webserver on {}", link);

  HttpServer::new(|| App::new().service(fs::Files::new("/", "./public/").index_file("index.html")))
    .bind((addr, port))?
    .run()
    .await
}
