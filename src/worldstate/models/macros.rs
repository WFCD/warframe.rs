#[macro_export]
macro_rules! model_builder {
    /*
    Creates an RTObject Model.

    Example:
    ```rs
    model_builder! {
        Cetus: "/cetusCycle";
        pub state: CetusState,
    }
    ```
    */
    ($struct_name:ident : $endpoint:literal, rt = obj; $($visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*$(,)?) => {
        $crate::impl_model_struct!(@basic $struct_name : $endpoint; $($visibility $field : $field_type $(= $rename:literal)?),*);
        $crate::impl_endpoint!($struct_name, $endpoint);

        impl $crate::ws::RTObject for $struct_name {}
    };


    /*
    Creates an RTObject + TimedEvent Model.

    Example:
    ```rs
    model_builder! {
        Cetus: "/cetusCycle",
        timed = true;
        pub state: CetusState,
    }
    ```
    */
    ($struct_name:ident : $endpoint:literal, rt = obj, timed = true; $($visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*$(,)?) => {
        $crate::impl_model_struct!(@timed $struct_name; $($visibility $field : $field_type $(= $rename:literal)?),*);
        $crate::impl_timed_event!($struct_name);
        $crate::impl_endpoint!($struct_name, $endpoint);

        impl $crate::ws::RTObject for $struct_name {}
    };


    /*
    Creates an RTArray Model.

    Example:
    ```rs
    model_builder! {
        Cetus: "/cetusCycle",
        rt_array = true;
        pub state: CetusState,
    }
    ```
    */
    ($struct_name:ident : $endpoint:literal, rt = array; $($visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*$(,)?) => {
        $crate::impl_model_struct!(@basic $struct_name; $($visibility $field : $field_type $(= $rename:literal)?),*);
        $crate::impl_endpoint!($struct_name, $endpoint);

        impl $crate::ws::RTArray for $struct_name {}
    };


    /*
    Creates an RTArray + TimedEvent Model.

    Example:
    ```rs
    model_builder! {
        Cetus: "/cetusCycle",
        rt_array = true,
        timed = true;
        pub state: CetusState,
    }
    ```
    */
    ($struct_name:ident : $endpoint:literal, rt = array, timed = true; $($visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*$(,)?) => {
        $crate::impl_model_struct!(@timed $struct_name; $($visibility $field : $field_type $(= $rename:literal)?),*);
        $crate::impl_timed_event!($struct_name);
        $crate::impl_endpoint!($struct_name, $endpoint);

        impl $crate::ws::RTArray for $struct_name {}
    };
}

// ---------------------------------
#[macro_export]
macro_rules! impl_model_struct {
    (@basic $struct_name:ident; $($visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*) => {
        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $struct_name {
            $(
                $(#[serde(rename(deserialize = $rename))])?
                $visibility $field : $field_type,
            )*
        }

        impl $crate::ws::Model for $struct_name {}
    };

    (@timed $struct_name:ident; $($visibility:vis $field:ident : $field_type:ty $(= $rename:literal)?),*) => {
        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $struct_name {
            $(
                $(#[serde(rename(deserialize = $rename))])?
                $visibility $field : $field_type,
            )*
            pub activation: chrono::DateTime<chrono::Utc>,
            pub expiry: chrono::DateTime<chrono::Utc>,
        }

        impl $crate::ws::Model for $struct_name {}
    };
}

// ---------------------------------
#[macro_export]
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
#[macro_export]
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
#[macro_export]
macro_rules! enum_builder {
    ($enum_name:ident; $($enum_option:ident $(= $enum_option_deserialize:literal)?),*$(,)?) => {
        #[derive(Debug, serde::Deserialize)]
        pub enum $enum_name {
            $(
                $(
                    #[serde(rename(deserialize = $enum_option_deserialize))]
                )?
                $enum_option,
            )*
        }
    };
}
