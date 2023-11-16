# warframe.rs

An async crate to wrap the [Worldstate API](https://docs.warframestat.us).

Use this crate if you want to make a Warframe-related rust project that is async.

## Getting started
No installation yet. You would have to clone this repo.

I will publish it to crates.io as soon as I have a CI configured.

### Example
```rs
use warframe::worldstate::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let client = Client::new();

    match client.fetch::<Cetus>().await {
        Ok(cetus) => {
            println!(
                "It is currently {} on cetus. It will be {} in {}",
                cetus.state,
                cetus.state.opposite(),
                cetus.eta()
            );
            Ok(())
        }
        Err(why) => Err(why),
    }
}
```

## Contributing
Contributions are more than welcome. To contribute simply fork this repository and make a PR.

I use `clippy` for linting, and `rustfmt` for formatting. They aree included as dev-dependencies in the `Cargo.toml`.

## - WIP -
This crate is work-in-progress and is meant as a learning project for me to learn Rust. This is basically a Rust adaptation of [warframe.py](https://github.com/WFCD/warframe.py).