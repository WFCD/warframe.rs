macro_rules! timed_event {
    ($struct_name:ident $($visibility:vis $field:ident : $field_type:ty),* ) => {
        #[derive(Debug, Deserialize)]
        pub struct $struct_name {
            $( $visibility $field : $field_type, )*
            pub activation: chrono::DateTime<chrono::Utc>,
            pub expiry: chrono::DateTime<chrono::Utc>,
        }

        impl super::base::Model for $struct_name {}
    }
}

pub(crate) use timed_event;
