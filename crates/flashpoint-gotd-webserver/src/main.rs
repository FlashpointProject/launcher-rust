use std::sync::{Arc, Mutex};

use actix_files as fs;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{cookie::Key, error, web, App, HttpRequest, HttpServer, Result};
use colored::Colorize;
use dotenv::dotenv;
use flashpoint_database::{models::Game, types::DbState};
use serde::{Deserialize, Serialize};

mod user;

#[derive(Deserialize, Serialize, Clone)]
pub struct SuggestionRequest {
  id: String,
  title: String,
  anonymous: bool,
  description: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SuggestionPublic {
  id: String,
  title: String,
  description: String,
  author: String,
}

#[derive(Deserialize, Serialize)]
pub struct SuggestionsResponse {
  suggestions: Vec<SuggestionPublic>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Suggestion {
  id: String,
  title: String,
  author: String,
  author_id: String,
  anonymous: bool,
  description: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct SuggestionWrapped {
  suggestion: Suggestion,
  ip_addr: String,
}

impl From<Suggestion> for SuggestionPublic {
  fn from(sug: Suggestion) -> Self {
    SuggestionPublic {
      id: sug.id,
      title: sug.title,
      description: sug.description,
      author: if sug.anonymous {
        "Anonymous".to_string()
      } else {
        sug.author
      },
    }
  }
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

async fn get_suggestions(
  sugs: web::Data<Arc<Mutex<Suggestions>>>,
) -> Result<web::Json<SuggestionsResponse>> {
  let sugs = sugs.lock().unwrap();
  let sugs = SuggestionsResponse {
    suggestions: sugs
      .suggestions
      .iter()
      .map(|s| s.suggestion.clone().into())
      .collect(),
  };
  Ok(web::Json(sugs))
}

async fn save_suggestion(
  sugs: web::Data<Arc<Mutex<Suggestions>>>,
  form: web::Json<SuggestionRequest>,
  id: Identity,
  session: Session,
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
        author: session
          .get::<String>("username")
          .map_err(|e| {
            error::ErrorInternalServerError(format!("Failed to get username from session: {}", e))
          })?
          .unwrap(),
        author_id: id.id().unwrap(),
        anonymous: form.anonymous,
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
  dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=debug");
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

  let secret_key = Key::generate();

  HttpServer::new(move || {
    App::new()
      .wrap(IdentityMiddleware::default())
      .wrap(SessionMiddleware::new(
        CookieSessionStore::default(),
        secret_key.clone(),
      ))
      .app_data(web::Data::new(db_arc.clone()))
      .app_data(web::Data::new(sugs_arc.clone()))
      .service(
        web::scope("/api")
          .service(
            web::scope("/auth")
              .route("/info", web::get().to(user::user_info))
              .route("/callback", web::get().to(user::callback))
              .route("/login", web::get().to(user::login))
              .route("/logout", web::get().to(user::logout)),
          )
          .route("/game/{gameId}", web::get().to(get_game))
          .route("/suggestion", web::post().to(save_suggestion))
          .route("/suggestions", web::get().to(get_suggestions)),
      )
      .service(fs::Files::new("/", "./public/").index_file("index.html"))
  })
  .bind((addr, port))?
  .run()
  .await
}
