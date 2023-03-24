use crate::models::moneylist::*;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}

// GET /money: สำหรับอ่านข้อมูลรายการรายรับ-รายจ่ายวันนี้
#[get("/money/today")]
async fn get_money_today(user_id: web::Json<UserdataUpgate>) -> impl Responder {
    info!("Keptang today");

    // ค่า id ที่รับมา
    let userdata = user_id.into_inner();
    let id: i32 = userdata.id;

    // ค่าเริ่มต้น
    let mut _user_money: i32 = 0;
    let mut _user_name = "";
    let mut _user_item = vec![];

    if id == 40956 {
        _user_money = 115000;
        _user_name = "vivat";
        _user_item = vec![
            Moneylist {
                list_id: 5,
                description: "เลี้ยงข้าวสาว".to_string(),
                date: "2023-03-15".to_string(),
                amount: 100,
                types: "expense".to_string(),
            },
            Moneylist {
                list_id: 4,
                description: "ซื้อข้าวเช้า".to_string(),
                date: "2023-03-15".to_string(),
                amount: 100,
                types: "expense".to_string(),
            },
            Moneylist {
                list_id: 3,
                description: "แม่ให้".to_string(),
                date: "2023-03-15".to_string(),
                amount: 300,
                types: "income".to_string(),
            },
        ];
    } else {
        _user_money = 0;
        _user_name = "";
    }
    // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
    #[derive(Serialize, Deserialize)]
    struct CombinedResponse {
        name: String,
        balance_total: i32,
        items: Vec<Moneylist>,
    }

    let combined_response = CombinedResponse {
        name: _user_name.to_string(),
        balance_total: _user_money,
        items: _user_item,
    };

    let response_body = json!(combined_response);

    HttpResponse::Ok().json(response_body)
}
