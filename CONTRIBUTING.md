# Tooling
We use the basic Rust tooling. Clippy, rustmft, etc.

If you have any recommendations regarding a  `rustfmt.toml`, please let us know/make a PR.

# Contributing
If you have a great idea for a feature, let us know [on the Discord](https://discord.gg/jGZxH9f).

Alternatively, you can open an issue.

# Project structure
To save you some time, here's a brief explanation of how this project is structured:

There are 2 modules for the "major" things you might want to do, that is querying the [worldstate](https://docs.warframestat.us) and the [market](https://warframe.market/api_docs) (with the `market` feature).

The `worldstate` module is much more developed. This is due to the market API getting a V2 soon.

Since the `market` module is rather small and easy to understand, we'll talk about the `worldstate` module.

## Worldstate module
All the models are defined via a function-like macro in the `worldstate/models` folder.
