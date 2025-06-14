use axum::Error;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub struct Settings{
    pub database_url:String,
    pub max_pool_connections:u8,
    pub addr:SocketAddr,
}

impl Settings{
    pub async fn new()->Result<Self, Error>{
        Ok(Self {
             database_url: String::from("postgresql://todolist_y7rd_user:fVKte93Keczk64tfgdF2dbCMkbAdBvZc@dpg-d0rh3s15pdvs73e072u0-a.oregon-postgres.render.com/todolist_y7rd?sslmode=require"),
              max_pool_connections:  40,
              addr:SocketAddr::from(([0, 0, 0, 0], 4002)),
        })
    }
}