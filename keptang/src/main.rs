use actix_web::{middleware, App, HttpServer, get, Responder, HttpResponse};
use env_logger::Env;
use serde::{Deserialize, Serialize};

mod handlers;
mod models;
pub mod routes;

use crate::routes::{
    money_routes,
    addmoney_routes,
    deletemoney_routes,
    editmoney_routes,
    profilemoney_routes,
    todaymoney_routes,
};

#[derive(Serialize, Deserialize, Clone)]
struct ListMoneyToday {
    list_id: i32,
    description: String,
    amount: i32,
    types: String,
}

static mut _user_balance_total: i32 = 0;
static mut _user_money_today: i32 = 0;
static mut _user_balance_income: i32 = 0;
static mut _user_balance_expense: i32 = 0;
static mut _date_today: &'static str = "";
static mut _user_name: &'static str = "";
static mut _user_item: Vec<ListMoneyToday> = vec![];
static mut _user_id: i32 = 0;


fn get_user_item() -> Vec<ListMoneyToday> {
    unsafe { _user_item.clone() }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Welcome to web-Keptang!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    
    unsafe {
        _user_balance_total = 12500;
        _user_name = "vivat";
        _user_id = 40956;
        _date_today = "2023-03-15";
        _user_item = vec![
            ListMoneyToday {
                list_id: 5,
                description: "เลี้ยงข้าวสาว".to_string(),
                amount: 100,
                types: "expense".to_string(),
            },
            ListMoneyToday {
                list_id: 4,
                description: "ซื้อข้าวเช้า".to_string(),
                amount: 100,
                types: "expense".to_string(),
            },
            ListMoneyToday {
                list_id: 3,
                description: "แม่ให้".to_string(),
                amount: 300,
                types: "income".to_string(),
            },            
        ];
        for item in &_user_item {
            if item.types == "income" {
                _user_balance_income += item.amount;
            } else if item.types == "expense" {
                _user_balance_expense += item.amount;
            }
        }
        _user_money_today = _user_balance_income - _user_balance_expense;
    }

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .configure(profilemoney_routes::config)
            .configure(money_routes::config)
            .configure(todaymoney_routes::config)
            .configure(addmoney_routes::config)
            .configure(editmoney_routes::config)
            .configure(deletemoney_routes::config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


