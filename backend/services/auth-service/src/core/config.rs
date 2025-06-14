use axum::Error;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub struct Settings{
    pub database_url:String,
    pub max_pool_connections:u8,
    pub addr:SocketAddr,
}

impl Settings{
    pub async fn load_config()->Result<Self, Error>{
        Ok(Settings {
             database_url: String::from("postgres://postgres:password@localhost/project1users"),
              max_pool_connections:  40,
              addr:SocketAddr::from(([0, 0, 0, 0], 4002)),
        })
    }
}