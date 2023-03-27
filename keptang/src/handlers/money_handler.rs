use crate::App;
use crate::HttpServer;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

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
    let mut _user_money_today: i32 = 0;
    let mut _user_money: i32 = 0;
    let mut _user_balance_income: i32 = 0;
    let mut _user_balance_expense: i32 = 0;
    let mut _user_name = "";
    let mut _user_item = vec![];
    let date_today = "2023-03-15";
    #[derive(Serialize, Deserialize)]
    struct ListMoneyToday {
        list_id: i32,
        description: String,
        amount: i32,
        types: String,
    }

    if id == 40956 {
        _user_money = 115000;
        _user_name = "vivat";
        _user_item = vec![
            ListMoneyToday {
                list_id: 5,
                description: "เลี้ยงข้าวสาว".to_string(),
                amount: 100,
                types: "expense".to_string(),
            },
            ListMoneyToday {
                list_id: 4,
                description: "ซื้อข้าวเช้า".to_string(),
                amount: 100,
                types: "expense".to_string(),
            },
            ListMoneyToday {
                list_id: 3,
                description: "แม่ให้".to_string(),
                amount: 300,
                types: "income".to_string(),
            },
            
        ];
        

        _user_balance_income = 0;
        _user_balance_expense = 0;
        for item in &_user_item {
            if item.types == "income" {
                _user_balance_income += item.amount;
            } else if item.types == "expense" {
                _user_balance_expense += item.amount;
            }
        }
        _user_money_today = _user_balance_income - _user_balance_expense;
    } else {
        _user_money = 0;
        _user_name = "";
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
        name: _user_name.to_string(),
        balance_total: _user_money,
        balance_today: _user_money_today,
        balance_income: _user_balance_income,
        balance_expense: _user_balance_expense,
        date: date_today.to_string(),
        items: _user_item,
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