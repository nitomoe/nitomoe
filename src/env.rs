// FIXME: Replace lazy_static

lazy_static::lazy_static! {
    pub static ref DATABASE_URL: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}