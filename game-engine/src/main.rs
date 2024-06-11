mod rabbit;

use crate::rabbit::lapin_listen;

#[tokio::main]
async fn main() {
    let mut cfg = deadpool_lapin::Config::default();
    cfg.url = Some("amqp://guest:guest@localhost:5672".into());
    let lapin_pool = cfg.create_pool(Some(deadpool_lapin::Runtime::Tokio1)).unwrap();
    lapin_listen(lapin_pool.clone()).await;
}
