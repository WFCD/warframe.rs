use std::{error::Error, sync::Arc};

use warframe::worldstate::prelude::*;

// Define some state
#[derive(Debug)]
struct MyState {
    _num: i32,
    _s: String,
}

async fn on_cetus_update(state: Arc<MyState>, before: &Cetus, after: &Cetus) {
    println!("STATE  : {state:?}");
    println!("BEFORE : {before:?}");
    println!("AFTER  : {after:?}");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let client = Client::new();

    // Note that the state will be cloned into the handler, so Arc is preferred
    let state = Arc::new(MyState {
        _num: 69,
        _s: "My ginormous ass".into(),
    });

    client
        .call_on_update_with_state(on_cetus_update, state)
        .await?;
    Ok(())
}
