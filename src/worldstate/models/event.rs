use super::{macros::model_builder, Faction, Reward, Syndicate};

model_builder! {
    :"An Event in Warframe"
    Event: "/events",
    rt = array,
    timed = true;

    :"Maximum score to complete the event"
    pub maximum_score: Option<i32>,

    :"The current score for the event"
    pub current_score: Option<i32>,

    :"Interval for the first goal"
    pub small_interval: Option<i32>,

    :"Interval for the second intermediate score"
    pub large_interval: Option<i32>,

    :"The faction you're up against"
    pub faction: Option<Faction>,

    :r"The description for the event"
    pub description: Option<String>,

    :"Tooltip for the event"
    pub tooltip: Option<String>,

    :"Node that the event is taking place on"
    pub node: Option<String>,

    :"Nodes that the event is happening concurrently on"
    pub concurrent_nodes: Vec<String>,

    :"Node that is being attacked & defended in the event"
    pub victim_node: Option<String>,

    :"Localized tag for the event score"
    pub score_loc_tag: Option<String>,

    :"The rewards to earn"
    pub rewards: Vec<Reward>,

    :"Amount of health remaining for the target"
    pub health: Option<i32>,

    :"The associated Syndicate"
    pub affiliated_with: Option<Syndicate>,
}

#[cfg(test)]
mod test {
    use super::Event;
    use crate::worldstate::{client::Client, error::ApiError};

    #[cfg(not(feature = "multilangual"))]
    #[tokio::test]
    async fn test_event() -> Result<(), ApiError> {
        let client = Client::new();

        match client.fetch_arr::<Event>().await {
            Ok(_events) => Ok(()),
            Err(why) => Err(why),
        }
    }

    #[cfg(feature = "multilangual")]
    #[tokio::test]
    async fn test_event_ml() -> Result<(), ApiError> {
        use crate::worldstate::prelude::Language;

        let client = Client::new();

        match client.fetch_arr_using_lang::<Event>(Language::ZH).await {
            Ok(_events) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
