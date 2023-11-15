macro_rules! model_builder {
    /*
    Creates an RTObject Model.

    Example:
    ```rs
    model_builder! {
        Cetus: "/cetusCycle",
        rt = obj;
        pub state: CetusState,
    }
    ```
    */
    ($(:$struct_doc:literal)? $struct_name:ident $(: $endpoint:literal)?, rt = obj; $($(:$field_doc:literal)? $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*$(,)?) => {
        $crate::ws::impl_model_struct!(@basic $(:$struct_doc)? $struct_name; $($(:$field_doc)? $visibility $field : $field_type $(= $rename:literal)?),*);
        $( $crate::ws::impl_endpoint!($struct_name, $endpoint); )?

        impl $crate::ws::RTObject for $struct_name {}
    };


    /*
    Creates an RTObject + TimedEvent Model.

    Example:
    ```rs
    model_builder! {
        Cetus: "/cetusCycle",
        rt = obj,
        timed = true;
        pub state: CetusState,
    }
    ```
    */
    ($(:$struct_doc:literal)? $struct_name:ident $(: $endpoint:literal)?, rt = obj, timed = true; $($(:$field_doc:literal)? $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*$(,)?) => {
        $crate::ws::impl_model_struct!(@timed $(:$struct_doc)? $struct_name; $($(:$field_doc)? $visibility $field : $field_type $(= $rename:literal)?),*);
        $crate::ws::impl_timed_event!($struct_name);
        $( $crate::ws::impl_endpoint!($struct_name, $endpoint); )?

        impl $crate::ws::RTObject for $struct_name {}
    };


    /*
    Creates an RTArray Model.

    Example:
    ```rs
    model_builder! {
        Cetus: "/cetusCycle",
        rt = array;
        pub state: CetusState,
    }
    ```
    */
    ($(:$struct_doc:literal)? $struct_name:ident $(: $endpoint:literal)?, rt = array; $($(:$field_doc:literal)? $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*$(,)?) => {
        $crate::ws::impl_model_struct!(@basic $(:$struct_doc)? $struct_name; $($(:$field_doc)? $visibility $field : $field_type $(= $rename:literal)?),*);
        $( $crate::ws::impl_endpoint!($struct_name, $endpoint); )?

        impl $crate::ws::RTArray for $struct_name {}
    };


    /*
    Creates an RTArray + TimedEvent Model.

    Example:
    ```rs
    model_builder! {
        Cetus: "/cetusCycle",
        rt = array,
        timed = true;
        pub state: CetusState,
    }
    ```
    */
    ($(:$struct_doc:literal)? $struct_name:ident $(: $endpoint:literal)?, rt = array, timed = true; $($(:$field_doc:literal)? $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*$(,)?) => {
        $crate::ws::impl_model_struct!(@timed $(:$struct_doc)? $struct_name; $($(:$field_doc)? $visibility $field : $field_type $(= $rename)?),*);
        $crate::ws::impl_timed_event!($struct_name);
        $( $crate::ws::impl_endpoint!($struct_name, $endpoint); )?

        impl $crate::ws::RTArray for $struct_name {}
    };
}

// ---------------------------------
macro_rules! impl_model_struct {
    (@basic $(:$struct_doc:literal)? $struct_name:ident; $($(:$field_doc:literal)? $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*) => {
        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        $(#[doc = $struct_doc])?
        pub struct $struct_name {
            $(
                $(#[serde(rename(deserialize = $rename))])?
                $(#[doc = $field_doc])?
                $visibility $field : $field_type,
            )*
        }

        impl $crate::ws::Model for $struct_name {}
    };

    (@timed $(:$struct_doc:literal)? $struct_name:ident; $($(:$field_doc:literal)? $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*) => {
        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        $(#[doc = $struct_doc])?
        pub struct $struct_name {
            $(
                $(#[serde(rename(deserialize = $rename))])?
                $(#[doc = $field_doc])?
                $visibility $field : $field_type,
            )*

            #[doc = "The time when the event began"]
            pub activation: chrono::DateTime<chrono::Utc>,

            #[doc = "The time when the event ends"]
            pub expiry: chrono::DateTime<chrono::Utc>,
        }

        impl $crate::ws::Model for $struct_name {}
    };
}

// ---------------------------------
macro_rules! impl_endpoint {
    ($struct_name:ident, $endpoint:literal) => {
        impl $crate::ws::Endpoint for $struct_name {
            fn endpoint() -> &'static str {
                concat!("https://api.warframestat.us/pc", $endpoint)
            }
        }
    };
}

// ---------------------------------
macro_rules! impl_timed_event {
    ($struct_name:ident) => {
        impl $crate::ws::TimedEvent for $struct_name {
            fn activation(&self) -> chrono::DateTime<chrono::Utc> {
                self.activation
            }

            fn expiry(&self) -> chrono::DateTime<chrono::Utc> {
                self.expiry
            }

            fn start_string(&self) -> String {
                $crate::ws::get_short_format_time_string(self.activation)
            }

            fn eta(&self) -> String {
                $crate::ws::get_short_format_time_string(self.expiry)
            }

            fn expired(&self) -> bool {
                chrono::Utc::now() >= self.expiry
            }
        }
    };
}

// ---------------------------------
macro_rules! enum_builder {
    ($(:$enum_doc:literal)? $enum_name:ident; $($(:$option_doc:literal)? $enum_option:ident $(= $enum_option_deserialize:literal)?),*$(,)?) => {
        #[derive(Debug, serde::Deserialize)]
        $(#[doc = $enum_doc])?
        pub enum $enum_name {
            $(
                $(
                    #[doc = $option_doc]
                )?
                $(
                    #[serde(rename(deserialize = $enum_option_deserialize))]
                )?
                $enum_option,
            )*
        }
    };
}

pub(crate) use enum_builder;
pub(crate) use impl_endpoint;
pub(crate) use impl_model_struct;
pub(crate) use impl_timed_event;
pub(crate) use model_builder;
