// URL Shortener in Rust using Actix-web and SQLite

// curl -X POST -H "Content-Type: application/json" -d "{\"original_url\": \"https://www.example.com\"}" http://127.0.0.1:8080/shorten

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection};
use std::sync::Mutex;
use nanoid::nanoid;

#[derive(Serialize, Deserialize)]
struct UrlPayload {
    original_url: String,
}

#[derive(Serialize)]
struct ShortenedUrl {
    shortened_url: String,
}

struct AppState {
    db_connection: Mutex<Connection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let conn = Connection::open("url_shortener.db").expect("Failed to connect to database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            id TEXT PRIMARY KEY,
            original_url TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create table");

    let state = web::Data::new(AppState {
        db_connection: Mutex::new(conn),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/shorten", web::post().to(shorten_url))
            .route("/{id}", web::get().to(redirect_url))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn shorten_url(data: web::Data<AppState>, payload: web::Json<UrlPayload>) -> impl Responder {
    let id = nanoid!(8);
    let original_url = &payload.original_url;

    {
        let conn = data.db_connection.lock().unwrap();
        conn.execute(
            "INSERT INTO urls (id, original_url) VALUES (?1, ?2)",
            params![id, original_url],
        ).expect("Failed to insert into database");
    }

    let shortened_url = format!("http://127.0.0.1:8080/{}", id);
    HttpResponse::Ok().json(ShortenedUrl { shortened_url })
}

async fn redirect_url(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let conn = data.db_connection.lock().unwrap();
    let mut stmt = conn.prepare("SELECT original_url FROM urls WHERE id = ?1").expect("Failed to prepare query");

    let original_url: Option<String> = stmt.query_row(params![id.as_str()], |row| row.get(0)).ok();

    if let Some(url) = original_url {
        HttpResponse::Found().append_header(("Location", url)).finish()
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}
