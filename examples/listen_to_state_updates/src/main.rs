use std::error::Error;

use warframe::worldstate::prelude::*;

async fn on_cetus_update(before: &Cetus, after: &Cetus) {
    println!("BEFORE : {before:?}");
    println!("AFTER  : {after:?}");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let client = Client::new();

    client.call_on_update(on_cetus_update).await?;
    Ok(())
}
