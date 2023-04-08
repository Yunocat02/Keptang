pub mod datareport;
use actix_web::{get,web, HttpResponse, Responder };
use log::info;
use mysql_async::Pool;
use serde::{Deserialize, Serialize};
use crate::HttpServer;
use crate::App;
use crate::handlers::profilemoney_handler::datareport::mainuser_money;
use std::collections::HashMap;
use firebase_rs::*;
use mysql_async::prelude::*;
use mysql::*;
use mysql::prelude::*;


// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize,Debug, PartialEq, Eq, Clone)]
struct UserdataUpgate {
    user_id: i32,
}


#[derive(Debug, PartialEq, Eq, Clone)]
struct Payment {
    user_id: i32,
}

// GET /profile 
#[get("/profile")]
async fn get_profile(id: web::Query<HashMap<String, String>>) -> impl Responder  {
    info!("Keptang_profile");

    
    let id_param = id.get("id");
    let id: i32 = match id_param {
        Some(val) => val.parse::<i32>().unwrap_or(0),
        None => 0,
    };
    let user_money = unsafe { mainuser_money };
    let mut user_name = "";
    let mut user_id: i32 = 0;

    if id == 40956 {
        user_name = "vivat";
        user_id = id;

    }

    #[derive(Serialize, Deserialize)]
    struct Response {
        name: String,
        balance_total: i32,
        id: i32,
    }

    let combined_response = Response {
        name: user_name.to_string(),
        balance_total: user_money,
        id: user_id,
    };
    let json_response = serde_json::to_string(&combined_response).unwrap();
    

    HttpResponse::Ok().json(json_response)
   
}



// fn แยก เรียก DB
fn get_database() -> Result<()> {
    let url = "mysql://root:@localhost:3306/test"; // URL ของ MySQL Server
    /*let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    // เรียกข้อมูลผู้ใช้งานจากตาราง user
    let user_query = "SELECT * FROM user WHERE id = 40956";
    let mut user_stmt = conn.prepare(user_query)?;
    let user_result = user_stmt.execute(()).unwrap();

    for row in user_result {
        let row = row.unwrap();
        let name: String = from_row(row);
        let balance_total: i32 = from_row(row);
        let id: i32 = from_row(row);
    }*/
    
    Ok(())
}