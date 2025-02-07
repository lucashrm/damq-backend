use std::collections::HashMap;
use std::error::Error;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

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

#[post("/api/token")]
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
            HttpResponse::InternalServerError().body("Failed to fetch token")
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(fetch_auth_token)
            .route("/hey", web::get().to(manual_hello))
    });

    server.bind(("127.0.0.1", 8080))?
        .run()
        .await.expect("Couldn't run server");

    Ok(())
}