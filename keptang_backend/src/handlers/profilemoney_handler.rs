use actix_web::{ get, HttpResponse, Responder, web};
use log::{info};
use serde::{Serialize, Deserialize};

// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}
// GET /profile 
#[get("/profile")]
async fn get_profile(user_id: web::Json<UserdataUpgate>) -> impl Responder {
    info!("Keptang profile");

    // ค่า id ที่รับมา
    let user_data = user_id.into_inner();
    let id: i32 = user_data.id;

    // ค่าเริ่มต้น
    let mut user_money: i32 = 0;
    let mut user_name = "";

    if id==40956 {
        user_money = 115000;
        user_name = "vivat";
    }
    // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
    #[derive(Serialize, Deserialize)]
    struct Response {
       name: String,
       balance_total: i32,
   }
   // ใส่ค่า ที่จะตอบกลับ
   let combined_response = Response {
        name: user_name.to_string(),
        balance_total: user_money,
    };

    HttpResponse::Ok().json(combined_response)
}