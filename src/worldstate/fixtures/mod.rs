pub mod alert;
pub mod arbitration;
pub mod archon_hunt;

#[macro_export]
macro_rules! fixture {
    ($fn_name:ident, $body:tt $(--- $body2:literal)?) => {
        paste::paste! {
            #[rstest::fixture]
            pub fn [<$fn_name _en>]() -> &'static str {
                $body
            }

            $(
                #[rstest::fixture]
                pub fn $fn_name () -> &'static str {
                    $body2
                }
            )?
        }
    };
}
