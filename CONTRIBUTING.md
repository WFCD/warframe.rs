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

### The `model_builder!` macro
For example, let's look at the definition for `Cetus`:
```rs
model_builder! {
    :"The Information about cetus"
    Cetus: "/cetusCycle",
    rt = obj,
    timed = true;

    :"The id of the cycle"
    pub id: String,

    :"The state of Cetus (day/night)"
    pub state: CetusState,
}
```
Doc strings are made using the `:"doc here"` syntax. Followed by the `TypeName: "/endpoint_url"`. Said endpoints get concatenated via 
```rs
concat!("https://api.warframestat.us/pc", $endpoint, "/?language=en")
```
at compile time. This prevents unnecessary allocations. Of course, this doesn't work when you want to query in another language.

When a type has this optional `: "/endpoint"`, it will implement the `Endpoint` trait like so:

```rs
impl Endpoint for $struct_name {
    fn endpoint_en() -> &'static str {
        concat!("https://api.warframestat.us/pc", $endpoint, "/?language=en")
    }

    
    fn endpoint(language: Language) -> String {
        format!(
            "https://api.warframestat.us/pc{}/?language={}",
            $endpoint,
            String::from(language)
        )
    }
}
```

This is followed by an `rt = obj/arr`, which tells the model in which format it is returned in.
For example, there are no more than 1 `Cetus` at a time, so the API responds with a single `Cetus` object, hence `rt = obj`. `Fissure`s on the other hand have multiple active at a time, so the API responds with an array of those fissures, hence on fissures it's `rt = arr`.

Next is `timed = true`. This is some trickery, because models who have this set to true will get 2 fields: `activation` and `expiry`, and will additionally implement the `TimedEvent` trait.

### Putting it all together
To understand this, lets look at the `Queryable` trait first:
```rs
pub trait Queryable: Endpoint {
    type Return: DeserializeOwned;
    fn query(
        request_executor: &reqwest::Client,
    ) -> impl std::future::Future<Output = Result<Self::Return, ApiError>> + Send {
        async {
            Ok(request_executor
                .get(Self::endpoint_en())
                .send()
                .await?
                .json::<Self::Return>()
                .await?)
        }
    }

    
    fn query_with_language(
        ...
}
```

if a model has the endpoint signature (`: "/endpoint"`), the `Queryable` trait will be implemented by the macro.
Based on the `rt`, the `type Return` will either be `Self`, or `Vec<Self>`.

Now, all the `Client`'s `fetch` does:
```rs
impl Client {
    pub async fn fetch<T>(&self) -> Result<T::Return, ApiError>
    where
        T: Queryable,
    {
        <T as Queryable>::query(&self.session).await
    }
}
```

This means, depending on the type queried, you get a `Vec<Model>`, or a single `Model`.

E.g.
```rs
let fissures: Vec<Fissure> = client.fetch<Fissure>().await?;
let cetus: Cetus = client.fetch<Cetus>().await?;
```

If you have any questions, feel free to ask on the discord, or open an issue.