use reqwest::Client;
use std::error::Error;
use sysinfo::{System};
use tokio::time::{sleep, Duration};

mod metrics_info;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut system = System::new_all();
    loop {
        system.refresh_all();
        system.refresh_cpu_all();

        let metrics=metrics_info::get_metrics(&system);

        send_metrics(&metrics).await?;
        sleep(Duration::from_secs(5)).await;
    }
}
  
async fn send_metrics(metrics: &metrics_info::Metrics) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
  //  println!("Sending metrics: {:?}", metrics);
    let response = client
        .post("http://localhost:8000/metrics")
        .json(metrics)
        .send()
        .await?;

    println!("Status: {}", response.status());

    Ok(())
}

    