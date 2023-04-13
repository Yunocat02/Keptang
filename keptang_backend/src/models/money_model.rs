use serde::{Serialize, Deserialize};
use crate::config::db::conDB;
use mysql::*;
use mysql::prelude::*;

// ตารางเก็บ บัญชีผู้ใช้  
#[derive(Serialize, Deserialize)]
pub struct user_data {
    pub user_name: String,
    pub balance_total: i32,
}
#[derive(Serialize, Deserialize,Clone)]
// เป็น object ที่เราสร้างขึ้นมา  ตารางเก็บ รายรับ-รายจ่าย  ใช้หน้า money
pub struct money_list {
    pub list_id: i32,
    pub description: String,
    pub date: String,
    pub amount: i32,
    pub types: String,
}

// money_input
#[derive(Serialize, Deserialize)]
pub struct money_request {
    pub id: i32
}

//money_output
#[derive(Serialize, Deserialize)]
pub  struct money_response {
    pub name: String,
    pub balance_total: i32,
    pub items: Vec<money_list>
}

// ไว้หา moneylist ที่จะเปลี่ยนค่า รวม
#[derive(Serialize, Deserialize)]
pub  struct money_response_byID {
    pub user_id: i32,
    pub amount: i32,
    pub types: String
}

// money-return-database
pub fn get_user_money(user_id:i32) -> Vec <money_response>{
    //get data from userdata table
    let db1 = conDB()
    .map(|mut conn| {
        conn.query_map(
            "SELECT  `user_name`, `balance_total` FROM `userdata` WHERE `user_id`= ".to_owned() + user_id.to_string().as_str(),
            |(user_name,balance_total)|{
            user_data
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
    //get data from money_list table 
    let db2 = conDB()
    .map(|mut conn| {
        conn.query_map(
            "SELECT moneylist.list_id,moneylist.description,moneylist.date,moneylist.amount,moneylist.types 
            FROM moneylist INNER JOIN userdata ON moneylist.user_id = userdata.user_id 
            WHERE moneylist.user_id =  ".to_owned() + user_id.to_string().as_str(),
            |(list_id,description,date,amount,types)|{
            money_list
                {
                    list_id,
                    description,
                    date,
                    amount,
                    types
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    //combine data1 and data2
    let mut data3: Vec<money_response> = Default::default();
    match (db1, db2) {
        (Ok(db1), Ok(db2)) => {
            for i in db1 {
                data3.push(money_response {
                    name: i.user_name.clone(),
                    balance_total: i.balance_total,
                    items: db2.clone(),
                });
            }
        }
        (Err(e), _) => println!("Error: {}", e),
        (_, Err(e)) => println!("Error: {}", e),
    }

    return data3;
}  

pub fn get_moneylist_byid(user_id:i32,list_id:i32) -> Vec<money_response_byID>{

    let sql_db = format!("SELECT moneylist.user_id,moneylist.amount,moneylist.types 
    FROM moneylist INNER JOIN userdata ON moneylist.user_id = userdata.user_id 
    WHERE moneylist.user_id =  {} AND moneylist.list_id = {}", user_id, list_id);

    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            sql_db,
            |(u_id,amount,types)|{
                money_response_byID
                {
                    user_id: u_id,
                    amount: amount,
                    types: types
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    let mut data:Vec <money_response_byID> = Default::default();
    match db {
        Ok(money) => {
            for i in money{
                data.push(i);
            }
        }
        Err(e) => println!("Error: {}", e)
    }
    return data;
}