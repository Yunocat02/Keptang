use serde::{Serialize, Deserialize};
use crate::config::db::conDB;
use mysql::*;
use mysql::prelude::*;
use chrono::{Local, DateTime};


// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่ต้องการ Request
#[derive(Serialize, Deserialize)]
pub struct today_request {
    pub user_id: i32,
}


#[derive(Serialize, Deserialize,Clone)]
pub struct today_response_item {
    pub list_id: i32,
    pub description: String,
    pub amount: i32,
    pub types: String,
}

#[derive(Serialize, Deserialize)]
pub struct user_today_response {
    pub user_name: String,
    pub balance_total: i32, 
}    
    // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
#[derive(Serialize, Deserialize)]
pub struct today_response {
    pub user_name: String,
    pub balance_total: i32,
    pub balance_today: i32,
    pub date: String,
    pub items: Vec<today_response_item>
}
// money-return-database
pub fn money_today(user_id:i32) -> Vec <today_response>{
    let time_now: DateTime<Local> = Local::now();
    let formatted = time_now.format("%Y-%m-%d").to_string();
    let sql_db1 = format!("SELECT  list_id, description, amount, types FROM moneylist WHERE user_id = {} AND date = '{}'", user_id, formatted);
    //get data from userdata table
    let db1 = conDB()
    .map(|mut conn| {
        conn.query_map(
            sql_db1,
            |(list_id,description,amount,types)| {
                today_response_item
                {
                    list_id,
                    description,
                    amount,
                    types     
                }
            }
        )
    })
   .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    //get data from money_list table 
    let db2 = conDB()
    .map(|mut conn| {
        conn.query_map(
            "SELECT  user_name, balance_total 
            FROM userdata WHERE user_id= ".to_owned() + user_id.to_string().as_str(),
            |(user_name,balance_total)| {
                user_today_response
                {
                    user_name,
                    balance_total       
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    //combine data1 and data2
    let mut data3: Vec<today_response> = Default::default();
    let mut db1_copy: Vec<today_response_item> = Default::default();
    let mut balance_today_calculator = 0;
    match (db1, db2) {
        (Ok(db1), Ok(db2)) => {
            
            db1_copy = db1.clone();
            for j in db1_copy{
                if j.types == "income"{
                    balance_today_calculator += j.amount;
                }else{
                    balance_today_calculator -= j.amount;
                }
            }

            for i in db2 {
                data3.push(today_response {
                    user_name: i.user_name.clone(),
                    balance_total: i.balance_total,
                    balance_today: balance_today_calculator,                 // ต้องคำนวณจริง 
                    date : formatted.to_string(),
                    items: db1.clone()
                });
            }
        }
        (Err(e), _) => println!("Error: {}", e),
        (_, Err(e)) => println!("Error: {}", e)
    }
    return data3;
} 
 


