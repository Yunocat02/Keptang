use crate::App;
use crate::HttpServer;
use crate::_user_balance_expense;
use crate::_user_balance_income;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;


use crate::{
    ListMoneyToday, _date_today, _user_balance_total, _user_item, _user_money_today, _user_name,
    get_user_item,
};
// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}
// GET /money: สำหรับอ่านข้อมูลรายการรายรับ-รายจ่ายวันนี้
#[get("/money")]
async fn get_money(id: web::Query<HashMap<String, String>>) -> impl Responder {
    info!("Keptang money");

    // ค่า id ที่รับมา
    let id_param = id.get("id");
    let id: i32 = match id_param {
        Some(val) => val.parse::<i32>().unwrap_or(0),
        None => 0,
    };

    // ค่าเริ่มต้น
    let mut user_name = String::new();
    let mut user_balance_total = 0;
    let mut user_money_today = 0;
    let mut date_today = String::new();
    let mut user_item = Vec::<ListMoneyToday>::new();
    let mut user_balance_income = 0;
    let mut user_balance_expense = 0;
    


    if id == 40956 {
        unsafe {
            user_name = _user_name.to_string();
            user_balance_total = _user_balance_total;
            user_money_today = _user_money_today;
            date_today = _date_today.to_string();
            user_item = _user_item.clone();
            user_balance_income = _user_balance_income;
            user_balance_expense = _user_balance_expense;
        }
    }
        
    // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
    #[derive(Serialize, Deserialize)]
    struct CombinedResponse {
        name: String,
        balance_total: i32,
        balance_today: i32,
        balance_income: i32,
        balance_expense: i32,
        date: String,
        items: Vec<ListMoneyToday>,
    }

    let combined_response = CombinedResponse {
        name: user_name,
        balance_total: user_balance_total,
        balance_today: user_money_today,
        balance_income: user_balance_income,
        balance_expense: user_balance_expense,
        date: date_today,
        items: user_item,
    };

    let response_body = serde_json::to_string(&combined_response).unwrap();
    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .content_type("application/json")
        .body(response_body)
    
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ค่าโฮสต์และพอร์ท
    let addr = "127.0.0.1:8080";

    // เปิดเซิร์ฟเวอร์
    let server = HttpServer::new(|| {
        App::new()
            .service(get_money)
    })
    .bind(addr)?;

    server.run().await
}