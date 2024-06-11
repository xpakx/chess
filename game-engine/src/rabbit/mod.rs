use std::time::Duration;

pub async fn lapin_listen(pool: deadpool_lapin::Pool) {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        println!("Connecting rmq consumer...");
        match init_lapin_listen(pool.clone()).await {
            Ok(_) => println!("RabbitMq listen returned"),
            Err(e) => println!("RabbitMq listen had an error: {}", e),
        };
    }
}

async fn init_lapin_listen(pool: deadpool_lapin::Pool) -> Result<(), Box<dyn std::error::Error>> {
    let rmq_con = pool.get().await
        .map_err(|e| {
        println!("Could not get RabbitMQ connnection: {}", e);
        e
    })?;
    let channel = rmq_con.create_channel().await?;

    // TODO

    let mut test_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        test_interval.tick().await;
        match channel.status().connected() {
            false => break,
            true => {},
        }
    }

    Ok(())
}
