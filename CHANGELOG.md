# [6.2.0](https://github.com/WFCD/warframe.rs/compare/v6.1.0...v6.2.0) (2024-10-28)


### Features

* damage types ([#20](https://github.com/WFCD/warframe.rs/issues/20)) ([ea8ddb1](https://github.com/WFCD/warframe.rs/commit/ea8ddb1ff1fd451ca7397d7529f9d1578b75170a))

# [6.1.0](https://github.com/WFCD/warframe.rs/compare/v6.0.1...v6.1.0) (2024-10-21)


### Features

* item query support and item types ([#19](https://github.com/WFCD/warframe.rs/issues/19)) ([f79f2d8](https://github.com/WFCD/warframe.rs/commit/f79f2d8dcff11f76867f0c0a938e2d78fa0f6529))

## [6.0.1](https://github.com/WFCD/warframe.rs/compare/v6.0.0...v6.0.1) (2024-08-27)


### Bug Fixes

* **docs:** added docs for almost everything ([2985fb1](https://github.com/WFCD/warframe.rs/commit/2985fb16e241d84bf4d476a35d34a28e4753af3d))

# [6.0.0](https://github.com/WFCD/warframe.rs/compare/v5.1.0...v6.0.0) (2024-07-19)


### Features

* simplified fetch to replace the array specific fetch ([3430874](https://github.com/WFCD/warframe.rs/commit/3430874f79f56b2cde691fdead25c87238495925))


### BREAKING CHANGES

* replaced fetch_arr, RT traits, and Model trait with the new Queryable trait

# [5.1.0](https://github.com/WFCD/warframe.rs/compare/v5.0.1...v5.1.0) (2024-07-11)


### Features

* market module ([#16](https://github.com/WFCD/warframe.rs/issues/16)) ([28dfebf](https://github.com/WFCD/warframe.rs/commit/28dfebf4bcf5d62f3258bd67e51536ae46fc4962))

## [5.0.1](https://github.com/WFCD/warframe.rs/compare/v5.0.0...v5.0.1) (2024-06-23)


### Bug Fixes

* syndicate jobs now have the type ([11b8dcd](https://github.com/WFCD/warframe.rs/commit/11b8dcdf3f02085c1a14cf870f55a7b85a8796f7))

# [5.0.0](https://github.com/WFCD/warframe.rs/compare/v4.1.0...v5.0.0) (2024-06-11)


### Bug Fixes

* listeners are now properly spawnable in a tokio::task::spawn(f) ([#13](https://github.com/WFCD/warframe.rs/issues/13)) ([d670259](https://github.com/WFCD/warframe.rs/commit/d6702592ae03306f610458bd02fbcc1d69671a4a))


### BREAKING CHANGES

* changed the signature of nested listener callback

# [4.1.0](https://github.com/WFCD/warframe.rs/compare/v4.0.0...v4.1.0) (2024-06-09)


### Features

* omnia fissures ([#11](https://github.com/WFCD/warframe.rs/issues/11)) ([1f01abe](https://github.com/WFCD/warframe.rs/commit/1f01abed71425043063fad17c7586a683cd693d9))

# [4.0.0](https://github.com/WFCD/warframe.rs/compare/v3.1.1...v4.0.0) (2024-03-05)


### Bug Fixes

* Invasion failing when InvasionMember is Infested ([#9](https://github.com/WFCD/warframe.rs/issues/9)) ([eff5b1a](https://github.com/WFCD/warframe.rs/commit/eff5b1a41aa3a67c5d900ed615d7c0d57c8fa62c))


### Features

* Added methods to make update listeners onto the client ([16bd7be](https://github.com/WFCD/warframe.rs/commit/16bd7be775613d85f2d1a753701d0316aabe0cbc))


### BREAKING CHANGES

* Deprecated warframe-macros and changed project structure

## [3.1.1](https://github.com/WFCD/warframe.rs/compare/v3.1.0...v3.1.1) (2024-02-10)


### Bug Fixes

* Fixed versioning issues ([b318c8d](https://github.com/WFCD/warframe.rs/commit/b318c8df7adbd21e26d3a3bca5503af3b2df8d71))

# [3.1.0](https://github.com/WFCD/warframe.rs/compare/v3.0.0...v3.1.0) (2024-02-10)


### Features

* optional States for listeners ([#6](https://github.com/WFCD/warframe.rs/issues/6)); ([236aa76](https://github.com/WFCD/warframe.rs/commit/236aa76ea836067eb0b80eb5028dee79f253ab6b))

# [3.0.0](https://github.com/WFCD/warframe.rs/compare/v2.0.0...v3.0.0) (2024-02-07)


### Features

* split warframe.rs and warframe-macros in monorepo ([#5](https://github.com/WFCD/warframe.rs/issues/5)) ([a1807b3](https://github.com/WFCD/warframe.rs/commit/a1807b3b5965034b62aa2da0c0e15b027e75a636))


### BREAKING CHANGES

* split where warframe.rs and warframe-macros are located in the repo

# [2.0.0](https://github.com/WFCD/warframe.rs/compare/v1.1.3...v2.0.0) (2024-02-05)


### chore

* Updates macros for better readability; ([35bed4c](https://github.com/WFCD/warframe.rs/commit/35bed4c4b9a229b108ee0ba53d2c185ad857ce64))


### BREAKING CHANGES

* Renamed the Enemy enum to Faction and most fields that use this type;

feat: Archon Hunt model;

feat: Cambion Drift Model;

docs: Changed docs for id field;

feat: Model for Construction Progress;

feat: Model for daily deal;

feat: Model for Event;

feat: Model for Flash Sale;

feat: Model for Global Upgrades (modifiers like double resources);

feat: Model for invasions;

updated mod.rs to include everything;

feat: Added Model for endpoints;

feat: added Model for Nightwave;

feat: added Model for Orb Vallis;

chore: Updated macro usage;

feat: added Model for Sortie;

feat: added Model for Steel Path;

feat: added Model for SyndicateMission;

feat: added Model for void trader;

feat: Macros for the project;

ci: Hopefully fix build error?;

ci: Fix build;

## [1.1.3](https://github.com/WFCD/warframe.rs/compare/v1.1.2...v1.1.3) (2024-01-28)


### Bug Fixes

* Typo in fetch_arr_using (changed to fetch_arr_using_lang) ([416fb3a](https://github.com/WFCD/warframe.rs/commit/416fb3ad42895c51ccf05b68c6ee31f4f4ac7911))

## [1.1.2](https://github.com/WFCD/warframe.rs/compare/v1.1.1...v1.1.2) (2024-01-27)


### Bug Fixes

* changed the multilangual feature to be less janky ([cc7c042](https://github.com/WFCD/warframe.rs/commit/cc7c042b973fb091542809bdd9216b7e4e5ec06a))

## [1.1.1](https://github.com/WFCD/warframe.rs/compare/v1.1.0...v1.1.1) (2024-01-26)


### Bug Fixes

* RewardType on Alerts is now being parsed correctly ([ef32c18](https://github.com/WFCD/warframe.rs/commit/ef32c18cf9d80161c3ac323584a93bdd5d2445ad))

# [1.1.0](https://github.com/WFCD/warframe.rs/compare/v1.0.4...v1.1.0) (2024-01-26)


### Bug Fixes

* Deserializing the `Mission` type is now working ([39d2672](https://github.com/WFCD/warframe.rs/commit/39d26723511bcd258713a87bf737823e7e604edc))
* Number -> Variant deserialization for `Tier` is now working (new dep however) ([5ec0e76](https://github.com/WFCD/warframe.rs/commit/5ec0e76e217c68f01073069a490bacabe416836c))


### Features

* arbitrations model ([470005e](https://github.com/WFCD/warframe.rs/commit/470005ec5518803b27c3a64e2c19d91d5ffc7249))
* Support for different languages using the new "multilangual" feature ([9e8460e](https://github.com/WFCD/warframe.rs/commit/9e8460ea60cecbf803b070e9bd0eaf62ed42bccf))

## [1.0.4](https://github.com/WFCD/warframe.rs/compare/v1.0.3...v1.0.4) (2023-11-16)


### Bug Fixes

* push Cargo.toml updates to git ([4deea46](https://github.com/WFCD/warframe.rs/commit/4deea46b5366df86321b2d5d2737e398c22d6fdb))

## [1.0.3](https://github.com/WFCD/warframe.rs/compare/v1.0.2...v1.0.3) (2023-11-16)


### Bug Fixes

* Cargo.toml ([0d29046](https://github.com/WFCD/warframe.rs/commit/0d290464370d843e83d46638d4eec43b4c7cd7ca))

## [1.0.2](https://github.com/WFCD/warframe.rs/compare/v1.0.1...v1.0.2) (2023-11-16)


### Bug Fixes

* Added required field for cargo publish ([cfede82](https://github.com/WFCD/warframe.rs/commit/cfede82a6416415ef850d4b4069907349453e835))

## [1.0.1](https://github.com/WFCD/warframe.rs/compare/v1.0.0...v1.0.1) (2023-11-16)


### Bug Fixes

* **docs:** Changed Contributing part ([f8d5ffa](https://github.com/WFCD/warframe.rs/commit/f8d5ffad93189600f3e31cadc1c9b65791e3af7b))

# 1.0.0 (2023-11-16)


### Bug Fixes

* warn ([c192e72](https://github.com/Mettwasser/warframe.rs/commit/c192e7292dc09298256641ef849f828cf90e079f))


### Features

* Alerts ([1634cf6](https://github.com/Mettwasser/warframe.rs/commit/1634cf67d34184e499ce20cca4f2a5f209526df1))
* **ci:** CI ([da41d90](https://github.com/Mettwasser/warframe.rs/commit/da41d909ad0851f301ae8940949ed2f6f41850ac))
* Test CI ([f7a8d9c](https://github.com/Mettwasser/warframe.rs/commit/f7a8d9c0d04bba2b95a1c5b5db75514f3c880b5c))
