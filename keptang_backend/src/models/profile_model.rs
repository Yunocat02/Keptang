use serde::{Serialize, Deserialize};
use crate::config::db::conDB;
use mysql::*;
use mysql::prelude::*;


// profile_input
#[derive(Serialize, Deserialize)]
pub struct profile_request {
    pub id: i32
}

//profile_output
#[derive(Serialize, Deserialize)]
pub struct profile_response {
    name: String,
    balance_total: i32,
    id: i32
}


// profile-return-database
pub fn get_user_profile(user_id:i32) -> Vec <profile_response>{
    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            "SELECT `user_id`, `user_name`, `balance_total` FROM `userdata` WHERE `user_id`= ".to_owned() + user_id.to_string().as_str(),
            |(id,name,balance_total)| {
             profile_response
                {
                    id,
                    name,
                    balance_total
                    
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
    let mut data:Vec <profile_response> = Default::default();
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
