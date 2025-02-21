macro_rules! model_builder {
    (
        $(:$struct_doc:literal)? $struct_name:ident $(: $endpoint:literal)?,
        rt = $rt:ident,
        timed = $timed:tt;
        $($(:$field_doc:literal)? $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)? $(=> $deserialize_func:literal)?),*$(,)?
    ) => {
        $crate::ws::impl_model_struct!(@timed = $timed $(:$struct_doc)? $struct_name; $($(:$field_doc)? $visibility $field : $field_type $(= $rename)? $(=> $deserialize_func)?),*);
        $crate::ws::impl_timed_event!($timed, $struct_name);
        $( $crate::ws::impl_queryable!($rt, $struct_name, $endpoint); )?



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
    (
        @timed = false :$struct_doc:literal $struct_name:ident;
        $(:$field_doc:literal $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)? $(=> $deserialize_func:literal)?),*) => {
        #[derive(Debug, serde::Deserialize, PartialEq, PartialOrd, Clone)]
        #[serde(rename_all = "camelCase")]
        #[doc = $struct_doc]
        pub struct $struct_name {
            $(
                $(#[serde(rename(deserialize = $rename))])?
                $(#[serde(deserialize_with = $deserialize_func)])?
                #[doc = $field_doc]
                $visibility $field : $field_type,
            )*
        }
    };

    (
        @timed = true :$struct_doc:literal $struct_name:ident;
        $(:$field_doc:literal $visibility:vis $field:ident : $field_type:ty $(= $rename:literal)? $(=> $deserialize_func:literal)?),*) => {
        #[derive(Debug, serde::Deserialize, PartialEq, PartialOrd, Clone)]
        #[serde(rename_all = "camelCase")]
        #[doc = $struct_doc]
        pub struct $struct_name {
            $(
                $(#[serde(rename(deserialize = $rename))])?
                $(#[serde(deserialize_with = stringify!($deserialize_func))])?
                #[doc = $field_doc]
                $visibility $field : $field_type,
            )*

            activation: chrono::DateTime<chrono::Utc>,

            expiry: chrono::DateTime<chrono::Utc>,
        }
    };
}

// ---------------------------------
macro_rules! impl_timed_event {
    (true, $struct_name:ident) => {
        impl $crate::ws::TimedEvent for $struct_name {
            #[doc = "Returns the time when this event began"]
            fn activation(&self) -> chrono::DateTime<chrono::Utc> {
                self.activation
            }

            #[doc = "Returns the time when this event ends"]
            fn expiry(&self) -> chrono::DateTime<chrono::Utc> {
                self.expiry
            }
        }
    };

    (false, $struct_name:ident) => {};
}

// ---------------------------------
macro_rules! enum_builder {
    ($(:$enum_doc:literal)? $enum_name:ident; $(:$option_doc1:literal)? $enum_option1:ident $(= $enum_option_deserialize1:literal)?, $(:$option_doc2:literal)? $enum_option2:ident $(= $enum_option_deserialize2:literal)? $(,)?) => {
        #[derive(Debug, serde::Deserialize, PartialEq, PartialOrd, Clone, Eq, Copy)]
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
        #[derive(Debug, serde::Deserialize, PartialEq, PartialOrd, Clone, Eq, Copy)]
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
        #[derive(Debug, serde_repr::Deserialize_repr, PartialEq, PartialOrd, Clone, Eq, Copy)]
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

macro_rules! impl_queryable {
    (array, $type:ty, $endpoint:literal) => {
        $crate::ws::impl_queryable!(@endpoint $type, $endpoint);
        impl $crate::ws::Queryable for $type {
            type Return = Vec<$type>;
        }
    };

    (obj, $type:ty, $endpoint:literal) => {
        $crate::ws::impl_queryable!(@endpoint $type, $endpoint);
        impl $crate::ws::Queryable for $type {
            type Return = $type;
        }
    };

    (@endpoint $struct_name:ty, $endpoint:literal) => {
        impl $crate::ws::Endpoint for $struct_name {
            fn endpoint_en() -> &'static str {
                concat!("https://api.warframestat.us/pc", $endpoint, "/?language=en")
            }

            
            fn endpoint(language: $crate::ws::Language) -> String {
                format!(
                    "https://api.warframestat.us/pc{}/?language={}",
                    $endpoint,
                    language
                )
            }
        }
    };
}

pub(crate) use enum_builder;
pub(crate) use impl_model_struct;
pub(crate) use impl_queryable;
pub(crate) use impl_timed_event;
pub(crate) use model_builder;
