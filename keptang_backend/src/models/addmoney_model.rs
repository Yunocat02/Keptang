use serde::{Serialize, Deserialize};
use crate::config::db::conDB;
use mysql::*;
use mysql::prelude::*;

// request user
#[derive(Serialize, Deserialize)]
pub struct add_request1 {
    pub user_id: i32
}
// request description,date,amount,types
#[derive(Serialize, Deserialize)]
pub struct add_request2 {
    pub description: String,
    pub date: String,
    pub amount: i32,
    pub types: String
}

// request all combine
#[derive(Serialize, Deserialize)]
pub struct add_request {
    pub user_data: add_request1,
    pub data_item: add_request2
}

//  add-return-database
pub fn insert_money(user_id:i32 , description:String,date:String,amount:i32,types:String){
    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "INSERT INTO moneylist ( description, date, amount, types, user_id) 
            VALUES ( :description , :date, :amount , :types, :user_id );",
            params! {
                "user_id" => user_id,
                "description" => description,
                "date" => date,
                "amount" =>amount,
                "types" =>types
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
}



    