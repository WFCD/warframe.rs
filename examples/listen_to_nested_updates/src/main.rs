use std::error::Error;

use warframe::worldstate::{listener::Change, prelude::*};

/// This function will be called once a fissure updates.
/// This will send a request to the corresponding endpoint once every 30s
/// and compare the results for changes.
async fn on_fissure_update(fissure: Change<'_, Fissure>) {
    match fissure {
        Change::Added(fissure) => println!("Fissure ADDED   : {fissure:?}"),
        Change::Removed(fissure) => println!("Fissure REMOVED : {fissure:?}"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Logging setup
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // initialize a client (included in the prelude)
    let client = Client::new();

    // Pass the function to the handler
    // (will return a Future)
    client.call_on_nested_update(on_fissure_update).await?;
    Ok(())
}
