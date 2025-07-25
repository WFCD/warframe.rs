# warframe.rs

An async crate to wrap the [Worldstate API](https://docs.warframestat.us) and the [warframe.market API](https://42bytes.notion.site/WFM-Api-v2-Documentation-5d987e4aa2f74b55a80db1a09932459d).

Use this crate if you want to make a Warframe-related rust project that is async.

## Getting started
To install, simply run `cargo add warframe`.

Note that the MSRV of this project is `1.85`.

### Example
```rust,no_run
use warframe::worldstate::{Client, Error, queryable::Cetus, Opposite, TimedEvent};

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
