//! Read listener_object first
use env_logger::builder as env_builder;
use warframe::worldstate::prelude::*;

/// Some arbitrary state
#[derive(Debug)]
struct SomeState {
    _message: String,
}

/// States specifiers allow you to pass a state to a handler of any listener.
/// The state is the SECOND parameter (comma-seperated) in listen(T). So listen(T, <State>)
///                                   vvvvvvvvv passed here
#[warframe::worldstate::listen(Cetus, SomeState)]
//                       vvvvv received as first argument
async fn on_cetus_update(state: &SomeState, before: &Cetus, after: &Cetus) {
    println!("BEFORE: {:?}", before);
    println!("AFTER: {:?}", after);
    println!("STATE: {:?}", state);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_builder().filter_level(log::LevelFilter::Debug).init();

    let client = Client::new();

    // create the state
    let state = SomeState {
        _message: "Hello, I'm a state!".into(),
    };

    // Of course, you still need to pass it to the handler
    on_cetus_update(&state, &client).await.unwrap();
    Ok(())
}

// Final Note: These work the same with listen_any(T, <State>)
