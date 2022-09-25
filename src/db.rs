use diesel::{
    pg::{PgConnection}, r2d2::{
        ConnectionManager, Pool, PoolError
    }
};
use crate::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn init_db_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_db_connection() -> PgPool {
    init_db_pool(&env::DATABASE_URL).expect("Failed to create database pool")
}