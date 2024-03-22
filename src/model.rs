use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub phone_number: String,
    pub password: String,
}
