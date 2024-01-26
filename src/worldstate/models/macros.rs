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

        $(
            impl $crate::ws::TypeDocumentation for $struct_name {
                fn docs() -> &'static str {
                    $struct_doc
                }
            }
        )?
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
        $crate::ws::impl_model_struct!(@timed $(:$struct_doc)? $struct_name; $($(:$field_doc)? $visibility $field : $field_type $(= $rename)?),*);
        $crate::ws::impl_timed_event!($struct_name);
        $( $crate::ws::impl_endpoint!($struct_name, $endpoint); )?

        impl $crate::ws::RTObject for $struct_name {}

        $(
            impl $crate::ws::TypeDocumentation for $struct_name {
                fn docs() -> &'static str {
                    $struct_doc
                }
            }
        )?
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

        $(
            impl $crate::ws::TypeDocumentation for $struct_name {
                fn docs() -> &'static str {
                    $struct_doc
                }
            }
        )?
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

        $(
            impl $crate::ws::TypeDocumentation for $struct_name {
                fn docs() -> &'static str {
                    $struct_doc
                }
            }
        )?
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

            activation: chrono::DateTime<chrono::Utc>,

            expiry: chrono::DateTime<chrono::Utc>,
        }

        impl $crate::ws::Model for $struct_name {}
    };
}

// ---------------------------------
macro_rules! impl_endpoint {
    ($struct_name:ident, $endpoint:literal) => {
        impl $crate::ws::Endpoint for $struct_name {
            #[cfg(not(feature = "multilangual"))]
            fn endpoint() -> &'static str {
                concat!("https://api.warframestat.us/pc/", $endpoint, "?language=en")
            }

            #[cfg(feature = "multilangual")]
            fn endpoint() -> &'static str {
                concat!("https://api.warframestat.us/pc/", $endpoint)
            }
        }
    };
}

// ---------------------------------
macro_rules! impl_timed_event {
    ($struct_name:ident) => {
        impl $crate::ws::TimedEvent for $struct_name {
            #[doc = "Returns the time when this event began"]
            fn activation(&self) -> chrono::DateTime<chrono::Utc> {
                self.activation
            }

            #[doc = "Returns the time when this event ends"]
            fn expiry(&self) -> chrono::DateTime<chrono::Utc> {
                self.expiry
            }

            #[doc = "Returns a formatted time string of when this event began"]
            fn start_string(&self) -> String {
                format!(
                    "-{}",
                    $crate::ws::get_short_format_time_string(self.activation)
                )
            }

            #[doc = "Returns a formatted time string of when this event ends"]
            fn eta(&self) -> String {
                $crate::ws::get_short_format_time_string(self.expiry)
            }

            #[doc = "Returns a bool stating whether the event has ended or not"]
            fn expired(&self) -> bool {
                chrono::Utc::now() >= self.expiry
            }
        }
    };
}

// ---------------------------------
macro_rules! enum_builder {
    ($(:$enum_doc:literal)? $enum_name:ident; $(:$option_doc1:literal)? $enum_option1:ident $(= $enum_option_deserialize1:literal)?, $(:$option_doc2:literal)? $enum_option2:ident $(= $enum_option_deserialize2:literal)? $(,)?) => {
        #[derive(Debug, serde::Deserialize, PartialEq)]
        $(#[doc = $enum_doc])?
        pub enum $enum_name {
            $(
                #[doc = $option_doc1]
            )?
            $(
                #[serde(rename(deserialize = $enum_option_deserialize1))]
            )?
            $enum_option1,
            $(
                #[doc = $option_doc2]
            )?
            $(
                #[serde(rename(deserialize = $enum_option_deserialize2))]
            )?
            $enum_option2,
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $enum_name::$enum_option1 => write!(f, stringify!($enum_option1)),
                    $enum_name::$enum_option2 => write!(f, stringify!($enum_option2)),
                }
            }
        }

        impl $crate::ws::VariantDocumentation for $enum_name {
            fn docs(&self) -> &'static str {
                match self {
                    $enum_name::$enum_option1 =>concat!("", $($option_doc1)?),
                    $enum_name::$enum_option2 => concat!("", $($option_doc2)?)
                }
            }
        }

        impl $crate::ws::TypeDocumentation for $enum_name {
            fn docs() -> &'static str {
                concat!("", $($enum_doc)?)
            }
        }

        impl $crate::ws::Opposite for $enum_name {
            fn opposite(&self) -> Self {
                match self {
                    $enum_name::$enum_option1 => $enum_name::$enum_option2,
                    $enum_name::$enum_option2 => $enum_name::$enum_option1
                }
            }
        }
    };

    ($(:$enum_doc:literal)? $enum_name:ident; $($(:$option_doc:literal)? $enum_option:ident $(= $enum_option_deserialize:literal)?),*$(,)?) => {
        #[derive(Debug, serde::Deserialize, PartialEq)]
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

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($enum_name::$enum_option => write!(f, stringify!($enum_option))),*
                }
            }
        }

        impl $crate::ws::VariantDocumentation for $enum_name {
            fn docs(&self) -> &'static str {
                match self {
                    $($enum_name::$enum_option => concat!("", $($option_doc)?)),*
                }
            }
        }

        impl $crate::ws::TypeDocumentation for $enum_name {
            fn docs() -> &'static str {
                concat!("", $($enum_doc)?)
            }
        }
    };

    ($(:$enum_doc:literal)? $enum_name:ident; $($(:$option_doc:literal)? $enum_option:ident $(= $enum_option_deserialize:literal)? $(: $enum_option_num_value:expr)?),*$(,)?) => {
        #[derive(Debug, serde_repr::Deserialize_repr, PartialEq)]
        #[repr(u8)]
        $(#[doc = $enum_doc])?
        pub enum $enum_name {
            $(
                $(
                    #[doc = $option_doc]
                )?
                $(
                    #[serde(rename(deserialize = $enum_option_deserialize))]
                )?
                $enum_option $(= $enum_option_num_value)?,
            )*
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($enum_name::$enum_option => write!(f, stringify!($enum_option))),*
                }
            }
        }

        impl $crate::ws::VariantDocumentation for $enum_name {
            fn docs(&self) -> &'static str {
                match self {
                    $($enum_name::$enum_option => concat!("", $($option_doc)?)),*
                }
            }
        }

        impl $crate::ws::TypeDocumentation for $enum_name {
            fn docs() -> &'static str {
                concat!("", $($enum_doc)?)
            }
        }
    }
}

pub(crate) use enum_builder;
pub(crate) use impl_endpoint;
pub(crate) use impl_model_struct;
pub(crate) use impl_timed_event;
pub(crate) use model_builder;
