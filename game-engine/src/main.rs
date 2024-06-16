mod rabbit;
mod config;
mod engine;

use crate::rabbit::lapin_listen;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let config = config::get_config();
    let mut cfg = deadpool_lapin::Config::default();
    cfg.url = Some(config.rabbit.into());
    let lapin_pool = cfg.create_pool(Some(deadpool_lapin::Runtime::Tokio1)).unwrap();
    lapin_listen(lapin_pool.clone()).await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    White,
    Red,
}
