use serde::{Serialize, Deserialize};
use crate::config::db::conDB;
use mysql::*;
use mysql::prelude::*;
use crate::models::money_model::*;
use log::{debug};

//request1edit
#[derive(Serialize, Deserialize)]
pub struct edit_request1 {
    pub user_id: i32
}

// request2edit
#[derive(Serialize, Deserialize)]
pub struct edit_request2 {
    pub description: String,
    pub date: String,
    pub amount: i32,
    pub types: String
}

// combine_request
#[derive(Serialize, Deserialize)]
pub struct edit_request {
    pub user_data: edit_request1,
    pub data_item: edit_request2
}

// response
#[derive(Serialize, Deserialize)]
struct edit_response {
    items_old: money_list,
    items_new: money_list,
    text: String,
}


//  edit-return-database
pub fn edit_money(user_id:i32 ,list_id:i32, description:String,date:String,amount:i32,types:String){
    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "UPDATE `moneylist` SET `description`= :description , `date`= :date, `amount`= :amount , `types`= :types  
            WHERE `list_id`= :list_id AND user_id = :user_id;",
            params! {
                "list_id" => list_id,
                "description" => description,
                "date" => date,
                "amount" =>amount,
                "types" =>types,
                "user_id" => user_id
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
}

//  แก้ไข balance_total หลัก
pub fn edit_balance_total(user_id:i32 ,balance_total_update:i32,types:String){
    
    if types.to_string() == "income"{
        let _ = match conDB() {
            Ok(mut conn) => {
                conn.exec_drop(
                "UPDATE `userdata` SET `balance_total`= `balance_total` + :balance_total_update WHERE `user_id` = :user_id",
                params! {
                    "balance_total_update" => balance_total_update,
                    "user_id" => user_id
                },
            )},
            Err(e) => {
                println!("Failed to get DB connection: {}", e);
                return;
            }
        };
    }else{
        let _ = match conDB() {
            Ok(mut conn) => {
                conn.exec_drop(
                "UPDATE `userdata` SET `balance_total`= `balance_total` - :balance_total_update WHERE `user_id` = :user_id",
                params! {
                    "balance_total_update" => balance_total_update,
                    "user_id" => user_id
                },
            )},
            Err(e) => {
                println!("Failed to get DB connection: {}", e);
                return;
            }
        };
    }
    
    
}