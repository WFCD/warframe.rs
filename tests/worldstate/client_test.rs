use warframe::worldstate::prelude::*;

#[tokio::test]
async fn test_fetch() -> Result<(), ApiError> {
    let client = Client::new();

    match client.fetch::<Cetus>().await {
        Ok(cetus) => {
            cetus.activation();
            cetus.expiry();
            cetus.start_string();
            cetus.expired();
            cetus.eta();
            Ok(())
        }
        Err(why) => Err(why),
    }
}

#[test]
fn test_docs() {
    assert_eq!(CetusState::Day.docs(), "Represents Cetus' day state");
    assert_eq!(
        <CetusState as TypeDocumentation>::docs(),
        "Represents the current state on cetus"
    );
}

#[tokio::test]
async fn test_fetch_arr() -> Result<(), ApiError> {
    let client = Client::new();

    match client.fetch_arr::<Fissure>().await {
        Ok(_fissures) => Ok(()),
        Err(why) => Err(why),
    }
}

#[tokio::test]
async fn test_alert() -> Result<(), ApiError> {
    let client = Client::new();

    match client.fetch_arr::<Alert>().await {
        Ok(_alerts) => Ok(()),
        Err(why) => Err(why),
    }
}
