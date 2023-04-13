use actix_web::{put, web, HttpResponse};
use log::{debug, info};
use crate::models::editmoney_model::*;


// PUT /money/edit/{id}: รับ JSON ที่มีค่าของคีย์ "expense"/"income" เป็นรายการ JSON ที่มีข้อมูลของรายการรายจ่ายที่ต้องการอัปเดตด้วย ID ที่ระบุ
#[put("/money/item/{id}")]
async fn put_money(list_id: web::Path<i32>,input_data: web::Json<EditRequest>) -> HttpResponse {
    info!("put money by id");
    debug!("id: {} 🪄", list_id);

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let user_data = input_data.into_inner();
    let id: i32 = list_id.to_string().parse().unwrap();

    edit_money(user_data.UserData.user_id,
        id,
        user_data.data_item.description,
        user_data.data_item.date,
        user_data.data_item.amount,
        user_data.data_item.types);


    HttpResponse::Ok().body("ทำการแก้ไขข้อมูลสำเร็จ👌")
}