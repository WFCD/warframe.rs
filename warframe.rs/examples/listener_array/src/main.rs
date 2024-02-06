use env_logger::builder as env_builder;
use warframe::worldstate::prelude::*;

/// Used to "listen" to updates of a Type T that implements `RTArray`.
/// The function gets passed `t: Change<'_, T>`
#[warframe::worldstate::listen_any(Fissure)]
async fn on_cetus_update(fissure: Change<'_, Fissure>) {
    match fissure {
        // Change<T> is an enum representing whether an item was added or removed
        Change::Added(fissure) => println!("A Fissure was just ADDED: {:?}", fissure),
        Change::Removed(fissure) => println!("A Fissure was just REMOVED: {:?}", fissure),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_builder().filter_level(log::LevelFilter::Debug).init();

    let client = Client::new();
    on_cetus_update(&client).await.unwrap();
    Ok(())
}
