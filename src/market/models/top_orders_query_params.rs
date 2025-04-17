use reqwest::RequestBuilder;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TopOrdersQueryParams {
    // INFO: Slug is required anyway, so it'll be passed as an argument
    // pub slug: Slug,
    pub rank: Option<u8>,
    pub rank_lt: Option<u8>,

    pub charges: Option<u8>,
    pub charges_lt: Option<u8>,

    pub amber_stars: Option<u8>,
    pub amber_stars_lt: Option<u8>,

    pub cyan_stars: Option<u8>,
    pub cyan_stars_lt: Option<u8>,

    pub subtype: Option<String>,
}

impl TopOrdersQueryParams {
    pub(crate) fn apply_to(self, mut request_builder: RequestBuilder) -> RequestBuilder {
        append_query!(request_builder, "rank", self.rank);
        append_query!(request_builder, "rank_lt", self.rank_lt);

        append_query!(request_builder, "charges", self.charges);
        append_query!(request_builder, "charges_lt", self.charges_lt);

        append_query!(request_builder, "amber_stars", self.amber_stars);
        append_query!(request_builder, "amber_stars_lt", self.amber_stars_lt);

        append_query!(request_builder, "cyan_stars", self.cyan_stars);
        append_query!(request_builder, "cyan_stars_lt", self.cyan_stars_lt);

        append_query!(request_builder, "subtype", self.subtype);

        request_builder
    }
}

macro_rules! append_query {
    ($builder_ident:ident, $name:literal, $option_value:expr) => {
        if let Some(value) = $option_value {
            $builder_ident = $builder_ident.query(&[($name, value)]);
        }
    };
}

use append_query;
