macro_rules! model_builder {
    ($struct_name:ident: $endpoint:literal $($visibility:vis $field:ident : $field_type:ty),*) => {
        #[derive(Debug, Deserialize)]
        pub struct $struct_name {
            $( $visibility $field : $field_type, )*
        }

        impl super::base::Model for $struct_name {}

        model_builder!(@endpoint $struct_name endpoint = $endpoint);
    };

    ($struct_name:ident : $endpoint:literal, timed = true; $($visibility:vis $field:ident : $field_type:ty),*) => {
        #[derive(Debug, Deserialize)]
        pub struct $struct_name {
            $( $visibility $field : $field_type, )*
            pub activation: chrono::DateTime<chrono::Utc>,
            pub expiry: chrono::DateTime<chrono::Utc>,

        }

        impl super::base::TimedEvent for $struct_name {
            fn start_string(&self) -> String {
                super::base::get_short_format_time_string(self.activation)
            }

            fn eta(&self) -> String {
                super::base::get_short_format_time_string(self.expiry)
            }

            fn expired(&self) -> bool {
                chrono::Utc::now() >= self.expiry
            }
        }

        impl super::base::Model for $struct_name {}

        model_builder!(@endpoint $struct_name endpoint = $endpoint);
    };

    (@endpoint $struct_name:ident endpoint = $endpoint:expr) => {
        lazy_static::lazy_static! {
            static ref ENDPOINT: String = format!("https://api.warframestat.us/pc{}", $endpoint);
        }

        impl super::base::Endpoint for $struct_name {
            fn endpoint() -> &'static str {
                &*ENDPOINT
            }
        }
    };
}

pub(crate) use model_builder;
