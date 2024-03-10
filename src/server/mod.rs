use crate::core::ShrewDB;
use ntex::web::{self};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static DB: Lazy<Mutex<ShrewDB<String, String>>> = Lazy::new(|| Mutex::new(ShrewDB::new()));

#[web::get("/ping")]
//                 ▼ this is an existential type.
async fn ping() -> impl web::Responder {
    web::HttpResponse::Ok().body("pong")
}

// TODO: Add /set/{key}

#[web::get("/get/{key}")]
async fn get(req: web::HttpRequest) -> impl web::Responder {
    let db = DB.lock().unwrap();
    let key: String = req.match_info().get("key").unwrap().parse().unwrap();
    let result = db.get(key);
    web::HttpResponse::Ok().body(format!("Searching for {:?}", result))
}

#[ntex::main]
pub async fn start() -> std::io::Result<()> {
    let endpoint = "127.0.0.1:6789";
    println!("Starting server at: {:?}", endpoint);
    web::HttpServer::new(|| web::App::new().service(ping).service(get))
        .bind(endpoint)?
        .run()
        .await
}
