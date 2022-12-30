use std::{
  env,
  sync::{Arc, Mutex},
};

use actix_files as fs;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{cookie::Key, error, web, App, HttpRequest, HttpServer, Responder, Result};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use colored::Colorize;
use dotenv::dotenv;
use flashpoint_database::{models::Game, types::DbState};
use fs::NamedFile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod user;

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGotdRequest {
  date: NaiveDate,
  suggestion: SuggestionPublic,
}
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
  game_id: String,
  title: String,
  description: String,
  date_submitted: NaiveDateTime,
  assigned_dates: Vec<NaiveDate>,
  author: String,
}

#[derive(Deserialize, Serialize)]
pub struct SuggestionsResponse {
  suggestions: Vec<SuggestionPublic>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Suggestion {
  id: String,
  game_id: String,
  title: String,
  author: String,
  author_id: String,
  date_submitted: NaiveDateTime,
  anonymous: bool,
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

#[derive(Deserialize, Serialize, Clone)]
struct GameOfTheDay {
  id: String,
  author: String,
  description: String,
  date: NaiveDate,
}

#[derive(Deserialize, Serialize, Clone)]
struct GameOfTheDayFile {
  games: Vec<GameOfTheDay>,
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
  gotds: web::Data<Arc<Mutex<GameOfTheDayFile>>>,
) -> Result<web::Json<SuggestionsResponse>> {
  let sugs = sugs.lock().unwrap();
  let gotds = gotds.lock().unwrap();
  let sugs = SuggestionsResponse {
    suggestions: sugs
      .suggestions
      .iter()
      .map(|s| {
        let dates: Vec<NaiveDate> = gotds
          .games
          .iter()
          .filter(|g| g.id == s.suggestion.game_id)
          .map(|g| g.date)
          .collect();
        SuggestionPublic {
          id: s.suggestion.id.clone(),
          game_id: s.suggestion.game_id.clone(),
          title: s.suggestion.title.clone(),
          author: s.suggestion.author.clone(),
          description: s.suggestion.description.clone(),
          date_submitted: s.suggestion.date_submitted,
          assigned_dates: dates,
        }
      })
      .collect(),
  };
  Ok(web::Json(sugs))
}

async fn index() -> impl Responder {
  NamedFile::open("public/index.html").unwrap()
}

async fn get_gotds(
  gotds: web::Data<Arc<Mutex<GameOfTheDayFile>>>,
) -> Result<web::Json<GameOfTheDayFile>> {
  let gotds = gotds.lock().unwrap();
  Ok(web::Json(gotds.clone()))
}

async fn delete_suggestion(
  session: Session,
  admin_ids: web::Data<Vec<String>>,
  sugs_id: web::Path<String>,
  sugs: web::Data<Arc<Mutex<Suggestions>>>,
  id: Identity,
) -> Result<web::Json<Suggestion>> {
  let user = user::get_user(session, id.id().unwrap(), admin_ids.to_vec())
    .await
    .map_err(|e| {
      error::ErrorInternalServerError(format!("Failed to get user from session: {}", e))
    })?;

  if !user.admin {
    return Err(error::ErrorForbidden("You are not an admin!"));
  }

  // remove suggestion with sugs_id id from suggestions
  let sugs_id = sugs_id.into_inner();
  let mut sugs = sugs.lock().unwrap();
  let index = sugs
    .suggestions
    .iter()
    .position(|s| s.suggestion.id == sugs_id)
    .ok_or(error::ErrorNotFound("Suggestion not found"))?;
  let sug = sugs.suggestions.remove(index);
  let sugs: Suggestions = Suggestions {
    suggestions: sugs.suggestions.clone(),
  };
  std::fs::write("./suggestions.json", serde_json::to_string(&sugs).unwrap())
    .map_err(|e| error::ErrorInternalServerError(e))?;

  Ok(web::Json(sug.suggestion))
}

async fn delete_gotd(
  session: Session,
  admin_ids: web::Data<Vec<String>>,
  date: web::Path<NaiveDate>,
  gotds: web::Data<Arc<Mutex<GameOfTheDayFile>>>,
  id: Identity,
) -> Result<web::Json<GameOfTheDay>> {
  let user = user::get_user(session, id.id().unwrap(), admin_ids.to_vec())
    .await
    .map_err(|e| {
      error::ErrorInternalServerError(format!("Failed to get user from session: {}", e))
    })?;

  if !user.admin {
    return Err(error::ErrorForbidden("You are not an admin!"));
  }

  // remove gotd with date from gotds
  let mut gotds = gotds.lock().unwrap();
  let date = date.into_inner();
  let index = gotds
    .games
    .iter()
    .position(|g| g.date == date)
    .ok_or(error::ErrorNotFound("Date not found"))?;
  let gotd = gotds.games.remove(index);
  let gotds = GameOfTheDayFile {
    games: gotds.games.clone(),
  };
  std::fs::write("./gotd.json", serde_json::to_string_pretty(&gotds).unwrap())
    .map_err(|e| error::ErrorInternalServerError(e))?;

  Ok(web::Json(gotd))
}

async fn set_gotd(
  db: web::Data<Arc<Mutex<DbState>>>,
  form: web::Json<SetGotdRequest>,
  gotds: web::Data<Arc<Mutex<GameOfTheDayFile>>>,
  id: Identity,
  session: Session,
  admin_ids: web::Data<Vec<String>>,
) -> Result<&'static str> {
  let user = user::get_user(session, id.id().unwrap(), admin_ids.to_vec())
    .await
    .map_err(|e| {
      error::ErrorInternalServerError(format!("Failed to get user from session: {}", e))
    })?;
  if !user.admin {
    return Err(error::ErrorForbidden("You are not an admin!"));
  }

  let mut db = db.lock().unwrap();
  let _ = flashpoint_database::game::find_game(&mut db, form.suggestion.game_id.clone())
    .map_err(|e| error::ErrorBadRequest(e))?;

  let mut gotds = gotds.lock().unwrap();
  if let Some(idx) = gotds.games.iter().position(|g| g.date == form.date) {
    gotds.games.remove(idx);
  }

  gotds.games.push(GameOfTheDay {
    id: form.suggestion.game_id.clone(),
    author: form.suggestion.author.clone(),
    description: form.suggestion.description.clone(),
    date: form.date,
  });
  let gotds = GameOfTheDayFile {
    games: gotds.games.clone(),
  };
  std::fs::write("./gotd.json", serde_json::to_string_pretty(&gotds).unwrap())
    .map_err(|e| error::ErrorInternalServerError(e))?;

  Ok("ok")
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
        id: Uuid::new_v4().to_string(),
        game_id: form.id.clone(),
        date_submitted: Utc::now().naive_utc(),
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
    std::fs::write("./suggestions.json", serde_json::to_string(&sugs).unwrap())
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

  let gotd_file = std::fs::read_to_string("./gotd.json").unwrap_or("{ \"games\": [] }".to_string());
  let gotds: GameOfTheDayFile = serde_json::from_str(&gotd_file).unwrap();
  let gotds_arc = Arc::new(Mutex::new(gotds));

  let admin_ids: Vec<String> = env::var("ADMIN_IDS")
    .unwrap_or(",".to_string())
    .split(',')
    .map(|s| s.to_string())
    .collect();

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
      .app_data(web::Data::new(gotds_arc.clone()))
      .app_data(web::Data::new(admin_ids.clone()))
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
          .route("/suggestion/{sugs_id}", web::delete().to(delete_suggestion))
          .route("/suggestions", web::get().to(get_suggestions))
          .route("/gotd/{date}", web::delete().to(delete_gotd))
          .route("/gotd", web::post().to(set_gotd))
          .route("/gotd", web::get().to(get_gotds)),
      )
      .route("/gotd", web::get().to(index))
      .route("/suggestions", web::get().to(index))
      .service(fs::Files::new("/", "./public/").index_file("index.html"))
  })
  .bind((addr, port))?
  .run()
  .await
}
