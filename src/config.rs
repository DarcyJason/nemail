use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub frontend_url: String,
    pub backend_url: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            frontend_url: var("FRONTEND_URL").expect("FRONTEND_URL must be set"),
            backend_url: var("BACKEND_URL").expect("BACKEND_URL must be set"),
            database_url: var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}
