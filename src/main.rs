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
