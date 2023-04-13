use serde::{Serialize, Deserialize};
use crate::config::db::conDB;
use mysql::*;
use mysql::prelude::*;


// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่ต้องการ Request
#[derive(Serialize, Deserialize)]
pub struct delete_request {
    pub user_id: i32,
}

// // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
// #[derive(Serialize, Deserialize)]
// pub struct delete_response {
//     pub items: money_list,
//     pub text: String,
// }

//  delete-database
pub fn delete_money_db(user_id:i32 ,list_id:i32){
    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "DELETE FROM moneylist 
            WHERE moneylist.list_id = :list_id 
            AND moneylist.user_id = :user_id",
            params! {
                "list_id" => list_id,
                "user_id" => user_id
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
}