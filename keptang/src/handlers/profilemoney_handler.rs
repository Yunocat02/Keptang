use actix_web::{get,web, HttpResponse, Responder };
use log::info;
use serde::{Deserialize, Serialize};
use crate::HttpServer;
use crate::App;
use std::collections::HashMap;

use crate::{
    ListMoneyToday, _user_balance_total, _user_item, _user_money_today, _user_name, _user_id,
    get_user_item,
};
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
    let mut user_name = String::new();
    let mut user_balance_total = 0;
    let mut user_id = 0;

    if id == 40956 {
        unsafe {
            user_name = _user_name.to_string();
            user_balance_total = _user_balance_total;
            user_id = _user_id
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Response {
        name: String,
        balance_total: i32,
        id: i32,
    }

    let combined_response = Response {
        name: user_name,
        balance_total: user_balance_total,
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