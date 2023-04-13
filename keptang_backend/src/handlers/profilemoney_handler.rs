use actix_web::{ get, HttpResponse, Responder, web};
use log::{info};
use serde::{Serialize, Deserialize};
use crate::models::profile_model::*;



// GET /profile 
#[get("/profile")]
async fn get_profile(user_id: web::Json<profile_request>) -> impl Responder {
    info!("Keptang profile");

    let userdata = user_id.into_inner();
    let id: i32 = userdata.id;  
    //ฟังก์ชันget_user_profile จาก src\models\moneylist.rs
    let data = get_user_profile(id);

   HttpResponse::Ok().json(data)
}