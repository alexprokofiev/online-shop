use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub phone_number: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub category_id: i64,
    pub desc: String,
    pub cost: f64,
    pub image: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct OrderProduct {
    pub order_id : i64,
    pub product_id: i64,
    pub quantity: i32,
}


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct CartProduct {
    pub product_id: i64,
    pub name: String,
    pub desc: String,
    pub image: String,
    pub cost: f64,
    pub order_id: i64,
    pub quantity: i32,
}

pub struct Order {
    pub id: i64,
    pub user_id: i64,
}