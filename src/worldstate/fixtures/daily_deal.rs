use crate::fixture;

fixture! {
    daily_deal,
r#"
[
  {
    "item": "Akbronco",
    "uniqueName": "/Lotus/StoreItems/Weapons/Tenno/Akimbo/AkimboShotGun",
    "expiry": "2025-02-24T19:00:00.000Z",
    "activation": "2025-02-23T17:00:00.000Z",
    "originalPrice": 225,
    "salePrice": 135,
    "total": 165,
    "sold": 5,
    "id": "AkimboShotGun1740423600000",
    "eta": "1d 1h 6m 19s",
    "discount": 40
  }
]
"#
---
r#"
[
  {
    "item": "野马双枪",
    "uniqueName": "/Lotus/StoreItems/Weapons/Tenno/Akimbo/AkimboShotGun",
    "expiry": "2025-02-24T19:00:00.000Z",
    "activation": "2025-02-23T17:00:00.000Z",
    "originalPrice": 225,
    "salePrice": 135,
    "total": 165,
    "sold": 5,
    "id": "AkimboShotGun1740423600000",
    "eta": "1d 1h 6m 29s",
    "discount": 40
  }
]
"#
}
