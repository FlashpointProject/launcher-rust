use std::sync::{Arc, Mutex};

use actix_files as fs;
use actix_web::{error, web, App, HttpRequest, HttpServer, Result};
use colored::Colorize;
use flashpoint_database::{models::Game, types::DbState};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SuggestionRequest {
  id: String,
  title: String,
  author: Option<String>,
  description: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Suggestion {
  id: String,
  title: String,
  author: String,
  description: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct SuggestionWrapped {
  suggestion: Suggestion,
  ip_addr: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct Suggestions {
  suggestions: Vec<SuggestionWrapped>,
}

async fn get_game(
  db: web::Data<Arc<Mutex<DbState>>>,
  game_id: web::Path<String>,
) -> Result<web::Json<Game>> {
  let mut db = db.lock().unwrap();
  let game = flashpoint_database::game::find_game(&mut db, game_id.into_inner())
    .map_err(|e| error::ErrorBadRequest(e))?;
  Ok(web::Json(game))
}

async fn save_suggestion(
  sugs: web::Data<Arc<Mutex<Suggestions>>>,
  form: web::Json<SuggestionRequest>,
  req: HttpRequest,
) -> Result<&'static str> {
  let conn_info = req.connection_info();
  let remote_ip = conn_info.realip_remote_addr();
  if let Some(val) = remote_ip {
    println!("Suggestion from {:?}", val);
    let mut sugs = sugs.lock().unwrap();
    sugs.suggestions.push(SuggestionWrapped {
      suggestion: Suggestion {
        id: form.id.clone(),
        title: form.title.clone(),
        description: form.description.clone(),
        author: form.author.clone().unwrap_or("Anonymous".to_string()),
      },
      ip_addr: val.to_string(),
    });
    let sugs: Suggestions = Suggestions {
      suggestions: sugs.suggestions.clone(),
    };
    std::fs::write(
      "./suggestions.json",
      serde_json::to_string_pretty(&sugs).unwrap(),
    )
    .map_err(|e| error::ErrorInternalServerError(e))?;
  }
  Ok("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let addr = "127.0.0.1";
  let port = 8080;
  let link = format!("http://{}:{}", addr, port).blue();
  println!("Starting webserver on {}", link);

  let sugs_file =
    std::fs::read_to_string("./suggestions.json").unwrap_or("{ \"suggestions\": [] }".to_string());
  let suggestions: Suggestions = serde_json::from_str(&sugs_file).unwrap();
  let sugs_arc = Arc::new(Mutex::new(suggestions));

  let db = flashpoint_database::initialize("./flashpoint.sqlite").unwrap();
  let db_arc = Arc::new(Mutex::new(db));

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(db_arc.clone()))
      .app_data(web::Data::new(sugs_arc.clone()))
      .service(
        web::scope("/api")
          .route("/game/{gameId}", web::get().to(get_game))
          .route("/suggestion", web::post().to(save_suggestion)),
      )
      .service(fs::Files::new("/", "./public/").index_file("index.html"))
  })
  .bind((addr, port))?
  .run()
  .await
}
