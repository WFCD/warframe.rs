# warframe.rs

An async crate to wrap the [Worldstate API](https://docs.warframestat.us).

Use this crate if you want to make a Warframe-related rust project that is async.

## Getting started
To install, simply run `cargo add warframe`.

### Example
```rust,no_run
use warframe::worldstate::{client::Client, error::Error, models::{Cetus, Opposite, TimedEvent}};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let cetus = client.fetch::<Cetus>().await?;
    println!(
        "It is currently {} on cetus. It will be {} in {}",
        cetus.state,
        cetus.state.opposite(),
        cetus.eta()
    );
    
    Ok(())
}
```

## Contributing
See [CONTRIBUTING](CONTRIBUTING.md)

### Commitlint

Commit messages are linting in the PR
