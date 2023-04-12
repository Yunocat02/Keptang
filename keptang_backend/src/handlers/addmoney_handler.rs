use actix_web::{web, HttpResponse, Responder,post};
use serde::{Serialize, Deserialize};
use log::{debug, info};
use serde_json::json;
use crate::models::addmoney_model::*;


// POST /money: สำหรับเพิ่มข้อมูลรายการรายรับรายจ่ายใหม่
#[post("/money/saving")]
async fn post_money(input_data: web::Json<add_request>) -> impl Responder {
    info!("post money");
    debug!("post: ✅");

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let user_data = input_data.into_inner();
    // function addmoney
    insert_money(user_data.user_data.user_id,
        user_data.data_item.description,
        user_data.data_item.date,
        user_data.data_item.amount,
        user_data.data_item.types);

    // HttpResponse::Ok().json(response_body)   ถ้าตัวนี้จะเป็น Status Code 200
    HttpResponse::Created().body("ทำการเพิ่มข้อมูลสำเร็จ👌") 

}
