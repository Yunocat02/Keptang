use actix_web::{delete, web, HttpResponse};
use log::{debug, info};
use crate::models::deletemoney_model::*;



// DELETE /money/delete/{id}: ไม่ต้องการรับข้อมูลใดๆ แต่จะลบรายการรายจ่ายที่มีอยู่ด้วย list_id ที่ระบุ
#[delete("/money/item/{id}")]
async fn delete_money(list_id: web::Path<i32>,input_data: web::Json<DeleteRequest>) -> HttpResponse {
    info!("delete money by list_id");
    debug!("list_id: {} ❌", list_id);

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let user_id = input_data.into_inner();
    let id: i32 = list_id.to_string().parse().unwrap();

    delete_money_db(user_id.user_id,id);


    HttpResponse::Ok().body("ทำการลบข้อมูลสำเร็จ👌")
}
