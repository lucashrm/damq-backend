use std::env;
use std::error::Error;
use std::sync::Mutex;
use diesel::prelude::*;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use damq_backend::models::users::{create_user, get_user};

struct AppState {
    conn: Mutex<MysqlConnection>
}

fn establish_connection() -> MysqlConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

   MysqlConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

#[derive(Deserialize, Serialize)]
struct TokenResponse {
    access_token: String
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("prout")
}

#[post("/token")]
async fn fetch_auth_token(req: String) -> impl Responder {
    println!("Incoming fetch with body {req}");
    dotenv().ok();
    let client_id = std::env::var("VITE_DISCORD_CLIENT_ID").expect("VITE_DISCORD_CLIENT_ID must be set");
    let client_secret = std::env::var("DISCORD_CLIENT_SECRET").expect("Discord secret must be set");

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("grant_type", "authorization_code".to_string()),
        ("code", req.clone()),
    ];

    println!("{params:?}");

    let client = reqwest::Client::new();
    let response = client.post("https://discord.com/api/oauth2/token")
        .form(&params)
        .send()
        .await;

    match response {
        Ok(res) => {
            match res.json::<TokenResponse>().await {
                Ok(token) => HttpResponse::Ok().json(token),
                Err(_) => HttpResponse::InternalServerError().body("Failed to parse response"),
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("{e}"))
        }
    }
}

#[post("/fetch_user")]
async fn fetch_user(req: String) -> impl Responder {
    let query = "query ($userName: String) {
      MediaListCollection(userName: $userName, type: ANIME) {
        lists {
          name
          entries {
            media {
              id
              title {
                romaji
                english
              }
              format
              status
            }
            score
            progress
            status
          }
        }
      }
    }";
    let client = Client::new();

    let json = json!({"query": query, "variables": {"userName": req.clone()}});

    let resp = client.post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await;

    println!("{:?}", resp);

    match resp {
        Ok(r) => {
            HttpResponse::Ok().body("")
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {e}"))
    }
}

#[post("/fetch_user_db")]
async fn fetch_user_db(req: String, data: web::Data<AppState>) -> impl Responder {
    let user_id = req.clone().parse::<i64>().unwrap();
    let mut conn = data.conn.lock().unwrap();

    let user = get_user(&mut conn, user_id);

    match user {
        Some(u) => {
            println!("{}", u.discord_id);
        }
        None => println!("no corresponding user")
    }

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let server = HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                conn: Mutex::from(establish_connection())
            }))
            .service(hello)
            .service(echo)
            .service(
                web::scope("/api")
                    .service(fetch_auth_token)
                    .service(fetch_user)
                    .service(fetch_user_db)
            )
            .route("/hey", web::get().to(manual_hello))
    });

    server.bind(("127.0.0.1", 8080))?
        .run()
        .await.expect("Couldn't run server");

    Ok(())
}