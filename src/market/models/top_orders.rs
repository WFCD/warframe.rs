use serde::Deserialize;

use super::order_with_user::OrderWithUser;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TopOrders {
    pub sell: Vec<OrderWithUser>,
    pub buy: Vec<OrderWithUser>,
}

#[cfg(test)]
mod test {
    use super::TopOrders;
    use crate::market::models::ResponseBase;

    #[rstest::rstest]
    fn lich_quirk(
        #[files("src/market/models/fixtures/top_orders.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<TopOrders>>(json)?;

        Ok(())
    }
}
