use anyhow::Result;
use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric, Uniform, Standard};

#[cfg(feature = "fix")]
use patch_fastly_api::apis::{healthcheck_api::{create_healthcheck, CreateHealthcheckParams}, configuration::{ApiKey, Configuration}};
#[cfg(not(feature = "fix"))]
use fastly_api::apis::{healthcheck_api::{create_healthcheck, CreateHealthcheckParams}, configuration::{ApiKey, Configuration}};

use std::env;

fn random_health_check_name() -> String {
    let mut rng = thread_rng();

    (&mut rng).sample_iter(Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut cfg = Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: env::var("FASTLY_API_TOKEN")?,
        }),
        ..Default::default()
    };

    let params = CreateHealthcheckParams {
        service_id: env::var("FASTLY_SERVICE_ID")?,
        version_id: env::var("FASTLY_SERVICE_VERSION")?.parse::<i32>()?, 
        headers: Some(vec![String::from("Ricky: Test"), String::from("Another-Header: Test")]),
        name: Some(random_health_check_name()),
        ..Default::default()
    };

    let res = create_healthcheck(&mut cfg, params).await;
    println!("Response:\n{res:?}\n");

    Ok(())
}
