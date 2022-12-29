use std::env;

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{error, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
  login_id: String,
  password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub username: String,
  pub admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordCode {
  code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordExchange {
  client_id: String,
  client_secret: String,
  grant_type: String,
  code: String,
  redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordToken {
  access_token: String,
  token_type: String,
  expires_in: i32,
  refresh_token: String,
  scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
  id: String,
  username: String,
  discriminator: String,
}

pub async fn get_user(session: Session, user_id: String, admin_ids: Vec<String>) -> Result<User> {
  let user = User {
    id: user_id.clone(),
    username: session
      .get::<String>("username")
      .unwrap_or(None)
      .unwrap_or("".to_string()),
    admin: if user_id != "" {
      admin_ids.contains(&user_id)
    } else {
      false
    },
  };
  Ok(user)
}

pub async fn user_info(
  session: Session,
  id: Identity,
  admin_ids: web::Data<Vec<String>>,
) -> Result<web::Json<User>> {
  let user = get_user(session, id.id().unwrap(), admin_ids.to_vec()).await?;
  Ok(web::Json(user))
}

pub async fn callback(
  req: HttpRequest,
  session: Session,
  query: web::Query<DiscordCode>,
) -> Result<HttpResponse> {
  let exchange = DiscordExchange {
    client_id: env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID not set"),
    client_secret: env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set"),
    grant_type: "authorization_code".into(),
    code: query.code.clone(),
    redirect_uri: env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI not set"),
  };

  // Exchange code for access token
  let exchange = serde_urlencoded::to_string(&exchange).expect("serialize issue");
  let client = reqwest::Client::new();
  let res = client
    .post("https://discord.com/api/oauth2/token")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .header("Accept", "application/json")
    .body(exchange)
    .send()
    .await
    .map_err(|e| error::ErrorBadRequest(e))?;
  let raw = res.text().await.map_err(|e| error::ErrorBadRequest(e))?;
  let token = serde_json::from_str::<DiscordToken>(&raw).map_err(|e| error::ErrorBadRequest(e))?;

  // Get user info
  let res = client
    .get("https://discord.com/api/users/@me")
    .header(
      "Authorization",
      format!("{} {}", token.token_type, token.access_token),
    )
    .send()
    .await
    .map_err(|e| error::ErrorBadRequest(e))?;
  let raw = res.text().await.map_err(|e| error::ErrorBadRequest(e))?;
  let json: serde_json::value::Value =
    serde_json::from_str(&raw).map_err(|e| error::ErrorBadRequest(e))?;
  let json = serde_json::to_string(&json).map_err(|e| error::ErrorBadRequest(e))?;
  let identity =
    serde_json::from_str::<DiscordUser>(&json).map_err(|e| error::ErrorBadRequest(e))?;

  // Store in session
  Identity::login(&req.extensions(), identity.id.clone()).expect("Failed to store identity");
  session.insert("username", identity.username.clone())?;

  Ok(
    HttpResponse::TemporaryRedirect()
      .append_header(("Location", "/"))
      .finish(),
  )
}

pub async fn login() -> impl Responder {
  HttpResponse::TemporaryRedirect()
    .append_header((
      "Location",
      format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify",
        env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID not set"),
        env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI not set")
      )
    ))
    .finish()
}

pub async fn logout(user: Identity, session: Session) -> impl Responder {
  user.logout();
  session.clear();
  HttpResponse::TemporaryRedirect()
    .append_header(("Location", "/"))
    .finish()
}
