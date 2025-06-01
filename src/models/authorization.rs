use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
}
#[derive(Serialize, FromRow, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
}