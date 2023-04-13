//use crate::models::moneylist::*;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use crate::models::todaymoney_model::*;

// GET /money: สำหรับอ่านข้อมูลรายการรายรับ-รายจ่ายวันนี้
#[get("/money/today")]
async fn get_money_today(user_id: web::Json<TodayRequest>) -> impl Responder {
    info!("Keptang today");

    // ค่า id ที่รับมา
    let userdata = user_id.into_inner();
    let user_id: i32 = userdata.user_id;

    let data = money_today(user_id);

    HttpResponse::Ok().json(data)
}
