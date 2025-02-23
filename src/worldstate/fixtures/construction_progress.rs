use crate::fixture;

fixture! {
    construction_progress,
r#"
{
  "id": "1740333060133100.50470218341587",
  "fomorianProgress": "100.50",
  "razorbackProgress": "73.86",
  "unknownProgress": "0.00"
}
"#
---
r#"
{
  "id": "1740333120099100.50470218341587",
  "fomorianProgress": "100.50",
  "razorbackProgress": "73.86",
  "unknownProgress": "0.00"
}
"#
}
