use crate::core::ShrewDB;
use colored::Colorize;
use ntex::web::{self};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static DB: Lazy<Mutex<ShrewDB<String, String>>> = Lazy::new(|| Mutex::new(ShrewDB::new()));

pub static ENDPOINT: &str = "127.0.0.1:6789";

#[web::get("/ping")]
//                 ▼ this is an existential type.
async fn ping() -> impl web::Responder {
    web::HttpResponse::Ok().body("PONG")
}

// TODO: Add POST /set/{key}

#[web::get("/get/{key}")]
async fn get(req: web::HttpRequest) -> impl web::Responder {
    let db = DB.lock().unwrap();
    let key: String = req.match_info().get("key").unwrap().parse().unwrap();
    let result = db.get(key);
    web::HttpResponse::Ok().body(format!("Searching for {:?}", result))
}

#[ntex::main]
pub async fn start() -> std::io::Result<()> {
    let welcome = r"
   ______ _____  _____      _____  ___ 
  / __/ // / _ \/ __| | /| / / _ \/ _ )
 _\ \/ _  / , _/ _/ | |/ |/ / // / _  |
/___/_//_/_/|_/___/ |__/|__/____/____/ 
                                       
";
    print!("{}", welcome.cyan());
    println!("starting shrewdb server at: {}", ENDPOINT.white().bold());
    web::HttpServer::new(|| web::App::new().service(ping).service(get))
        .bind(ENDPOINT)?
        .run()
        .await
}
