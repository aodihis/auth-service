use std::sync::Arc;
use sqlx::PgPool;
use crate::config::Config;

pub struct Permissions {
    pool: PgPool,
    config: Arc<Config>,
}

impl Permissions {
    pub fn new(pool: PgPool, config: Arc<Config>) -> Self {
        Permissions { pool, config }
    }

}
