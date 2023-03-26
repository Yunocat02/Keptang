pub mod datareport;
use actix_web::{get,web, HttpResponse, Responder };
use log::info;
use serde::{Deserialize, Serialize};
use crate::HttpServer;
use crate::App;
use crate::handlers::profilemoney_handler::datareport::mainuser_money;
use std::collections::HashMap;


// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}

// GET /profile 
#[get("/profile")]
async fn get_profile(id: web::Query<HashMap<String, String>>) -> impl Responder {
    info!("Keptang_profile");


    let id_param = id.get("id");
    let id: i32 = match id_param {
        Some(val) => val.parse::<i32>().unwrap_or(0),
        None => 0,
    };
    let user_money = unsafe { mainuser_money };
    let mut user_name = "";
    let mut user_id: i32 = 0;

    if id == 40956 {
        user_name = "vivat";
        user_id = id;

    }

    #[derive(Serialize, Deserialize)]
    struct Response {
        name: String,
        balance_total: i32,
        id: i32,
    }

    let combined_response = Response {
        name: user_name.to_string(),
        balance_total: user_money,
        id: user_id,
    };
    let json_response = serde_json::to_string(&combined_response).unwrap();

    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Methods", "GET, OPTIONS")
        .body(json_response)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ค่าโฮสต์และพอร์ท
    let addr = "127.0.0.1:8080";

    // เปิดเซิร์ฟเวอร์
    let server = HttpServer::new(|| {
        App::new()
            .service(get_profile)
    })
    .bind(addr)?;

    server.run().await
}