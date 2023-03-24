use actix_web::{web, HttpResponse, Responder,post};
use serde::{Serialize, Deserialize};
use log::{debug, info};
use serde_json::json;

use crate::models::moneylist::*;

// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่ต้องการ Request
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}
#[derive(Serialize, Deserialize)]
struct MoneylistUpgate {
    description: String,
    date: String,
    amount: i32,
    types: String,
}
// รวม โครงสร้าง JSON ที่จะเป็น Request
#[derive(Serialize, Deserialize)]
struct PostMoneyRequest {
    user_data: UserdataUpgate,
    data_item: MoneylistUpgate,
}
// POST /money: สำหรับเพิ่มข้อมูลรายการรายรับรายจ่ายใหม่
#[post("/money/new")]
async fn post_money(input_data: web::Json<PostMoneyRequest>) -> impl Responder {
    info!("post money");
    debug!("post: ✅");

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let user_data = input_data.into_inner();

    // สมมุติ เลข list_id ถ้าใช้จริง ก็ต่อจาก database
    let list_id_new = 6 ;

    // ชื่อ ผู้ใช้
    let mut user_name = "";

    // สมมุติข้อมูลใหม่ โดย list_id สร้างขึ้นมาเพิ่ม 
    let item_data = Moneylist {
        list_id: list_id_new,       // คือ list_id ที่ สร้างมาใหม่
        description: user_data.data_item.description,
        date: user_data.data_item.date,
        amount: user_data.data_item.amount,
        types: user_data.data_item.types,
    };

    // สมมุติ เป็นข้อมูลของ vivat
    if user_data.user_data.id == 40956{
        user_name = "vivat";
    }

    // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
     #[derive(Serialize, Deserialize)]
     struct CombinedResponse {
         u_name: String,
         items: Moneylist,
         text: String,
     }
 
     let combined_response = CombinedResponse {
         u_name: user_name.to_string(),
         items: item_data,
         text: "ทำการเพิ่มข้อมูลละเด้อ".to_string(),
     };
     
     let response_body = json!(combined_response);

    // HttpResponse::Ok().json(response_body)   ถ้าตัวนี้จะเป็น Status Code 200
     HttpResponse::Created().json(response_body)    // ถ้าตัวนี้จะเป็น Status Code 201

}
