use reqwest::Client;
use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Serialize)]
struct Metrics {
    cpu: f32,
    ram: u64,
}

#[tokio::main]
async fn main() {
    let mut system = System::new_all();

    system.refresh_all();

    let metrics = Metrics {
        cpu: system.global_cpu_usage(),
        ram: system.used_memory(),
    };
    loop{
    send_metrics(&metrics).await;
     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
  
}

async fn send_metrics(metrics: &Metrics){
  let client = Client::new();

    let response = client
        .post("http://localhost:3000/metrics")
        .json(metrics)
        .send()
        .await
        .unwrap();

    println!("Status: {}", response.status());
}