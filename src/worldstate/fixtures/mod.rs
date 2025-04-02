pub mod alert;
pub mod arbitration;
pub mod archon_hunt;
pub mod cambion_drift;
pub mod cetus;
pub mod construction_progress;
pub mod daily_deal;
pub mod deep_archimedea;
pub mod event;
pub mod fissure;
pub mod flash_sale;
pub mod global_upgrade;
pub mod invasion;
pub mod item;
pub mod news;
pub mod nightwave;
pub mod orb_vallis;
pub mod sortie;
pub mod steel_path;
pub mod syndicate_mission;
pub mod void_trader;
pub mod weapon;

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
