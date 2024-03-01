use std::{error::Error, fs::File};

use warframe::worldstate::{listener::Change, prelude::*};

async fn on_fissure_update(fissure: Change<'_, Fissure>) {
    match fissure {
        Change::Added(fissure) => println!("Fissure ADDED   : {fissure:?}"),
        Change::Removed(fissure) => println!("Fissure REMOVED : {fissure:?}"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let client = Client::new();

    client.call_on_any_update(on_fissure_update).await?;
    Ok(())
}
