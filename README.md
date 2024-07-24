# warframe.rs

An async crate to wrap the [Worldstate API](https://docs.warframestat.us).

Use this crate if you want to make a Warframe-related rust project that is async.

Please Note that a few things are subject to change without bumps in majors. Obsolete versions will be yanked asap.

## Getting started
To install, simply run `cargo add warframe`.

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

### Commitlint

Commit messages are linting in the PR
