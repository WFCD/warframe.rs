//! Fissure has the marker trait `RTArray`, meaning the API
//! responds with a list of Fissure objects.
//! To be able to query them, you use `fetch_arr<T>` instead.
//! This will give you a Vec<T>
use warframe::worldstate::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let client = Client::new();

    match client.fetch_arr::<Fissure>().await {
        Ok(fissures) => {
            fissures.iter().for_each(|f| println!("{f:?}"));
            Ok(())
        }
        Err(why) => Err(why),
    }
}
