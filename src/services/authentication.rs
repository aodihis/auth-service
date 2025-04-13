use sqlx::{AnyPool};
use crate::models::request::RegisterUser;

pub struct Authentication {
    pool: AnyPool,
}

impl Authentication {
    pub fn new(pool: AnyPool) -> Self {
        Self {
            pool
        }
    }

    pub fn register(&self, payload: RegisterUser) -> Result<(), String> {

        Ok(())
    }
}