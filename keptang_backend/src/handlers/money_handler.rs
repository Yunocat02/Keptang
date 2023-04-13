use actix_web::{ get, HttpResponse, Responder, web};
use log::{ info};
use serde_json::json;
use serde::{Serialize, Deserialize};
use crate::models::money_model::*;



// GET /money: สำหรับอ่านข้อมูลรายการรายรับรายจ่ายทั้งหมด
#[get("/money")]
async fn get_money(user_id: web::Json<money_request>) -> impl Responder {
    info!("Keptang money");

    // ค่า id ที่รับมา
    let userdata = user_id.into_inner();
    let id: i32 = userdata.id;
    //ฟังก์ชันget_user_money จาก src\models\moneylist.rs
    let data = get_user_money(id);

    HttpResponse::Ok().json(data)
}


