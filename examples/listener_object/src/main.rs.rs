#![cfg(feature = "macros")]
use env_logger::builder as env_builder;
use warframe::worldstate::prelude::*;

/// Used to "listen" to updates of a Type T that implements `RTObject`.
/// The function gets passed `before: &T, after: &T`
#[warframe::worldstate::listen(Cetus)]
async fn on_cetus_update(before: &Cetus, after: &Cetus) {
    println!("BEFORE: {:?}", before);
    println!("AFTER: {:?}", after);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_builder().filter_level(log::LevelFilter::Debug).init();

    let client = Client::new();

    // Note: The attribute changes the function signature to accept the client object for ease-of-use.
    // However, a proper way to "register" listeners on the client object is planned.
    on_cetus_update(&client).await.unwrap();
    Ok(())
}

#[cfg(not(feature = "macros"))]
fn main() {}
