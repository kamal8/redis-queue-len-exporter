use std::env;
use warp::Filter;
use prometheus::{Gauge, TextEncoder, Encoder};
use redis::aio::Connection;

async fn create_redis_client() -> redis::RedisResult<Connection> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1/"));
    let client = redis::Client::open(redis_url)?;
    client.get_async_connection().await
}

use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Get the queue names from the QUEUES environment variable
    let queues: Vec<String> = env::var("QUEUES")
        .unwrap_or_else(|_| String::from("default"))
        .split(',')
        .map(|s| s.to_string())
        .collect();

    // Create a new Gauge metric for each queue
    let mut gauges: HashMap<String, Gauge> = HashMap::new();
    for queue in &queues {
        let gauge = Gauge::new(format!("{}_gauge", queue), format!("A gauge for {} queue", queue)).unwrap();
        prometheus::register(Box::new(gauge.clone())).unwrap();
        gauges.insert(queue.clone(), gauge);
    }

    // Serve the metrics using Warp
    let metrics_route = warp::path!("metrics").and_then(move || {
        let gauges = gauges.clone();
        let queues = queues.clone();
        async move {
            let mut conn = create_redis_client().await.unwrap();
            for queue in &queues {
                let llen: f64 = redis::cmd("LLEN").arg(queue).query_async(&mut conn).await.unwrap();
                if let Some(gauge) = gauges.get(queue) {
                    gauge.set(llen);
                }
            }

            let encoder = TextEncoder::new();
            let metric_families = prometheus::gather();
            let mut buffer = vec![];
            encoder.encode(&metric_families, &mut buffer).unwrap();
            Ok::<_, warp::Rejection>(warp::reply::with_header(buffer, "Content-Type", encoder.format_type()))
        }
    });

    warp::serve(metrics_route).run(([0, 0, 0, 0], 8000)).await;
}