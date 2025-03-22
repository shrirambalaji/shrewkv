use crate::{core::ShrewKV, utils::parse_key_from_request};
use colored::Colorize;
use ntex::web::{self};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

static DB: Lazy<Mutex<ShrewKV<String, String>>> = Lazy::new(|| Mutex::new(ShrewKV::new()));

pub static ENDPOINT: &str = "127.0.0.1:6789";

#[web::get("/ping")]
//                 ▼ this is an existential type.
async fn ping() -> impl web::Responder {
    web::HttpResponse::Ok().body("PONG")
}

#[derive(Deserialize, Serialize)]
pub struct SetRequestBody {
    pub value: String,
}

#[web::get("/get/{key}")]
async fn get(req: web::HttpRequest) -> impl web::Responder {
    dbg!(&req);
    let db = DB.lock().unwrap();
    // let key: String = parse_key_from_request(&req);
    let key: String = parse_key_from_request(&req);
    let result = db.get(key.clone());
    match result {
        Some(value) => web::HttpResponse::Ok().json(&value),
        None => web::HttpResponse::NotFound().body("not found"),
    }
}

#[web::put("/set/{key}")]
async fn set(req: web::HttpRequest, body: web::types::Json<SetRequestBody>) -> impl web::Responder {
    dbg!(&req);
    let mut db = DB.lock().unwrap();
    let key = parse_key_from_request(&req);
    // is there a better way to do this?
    let result = db.put(key, body.value.to_string());
    web::HttpResponse::Ok().body(format!("Setting {:?}", result))
}

#[ntex::main]
pub async fn start() -> std::io::Result<()> {
    let welcome = r"
   ShrewKV 
";
    print!("{}", welcome.cyan());
    println!("starting ShrewKV server at: {}", ENDPOINT.white().bold());
    // TODO: handle error when PORT is busy.
    web::HttpServer::new(|| web::App::new().service(ping).service(get))
        .bind(ENDPOINT)?
        .run()
        .await
}
