# Changelog
All notable changes to this project will be documented in this file.    
The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.9.1] - 2019-02-12
### Added
- `#[derive(Debug)]` for `APIClient`

## [0.9.0] - 2018-12-07
### Changed
- Rust 2018 :tada:

## [0.8.0] - 2018-11-24
### Changed
- Updated to API version **5.92**.

## [0.7.0] - 2018-10-27
### Added
- `messages.getRecentCalls` method.

### Changed
- Improved performance using `lazy_static!`.
- Updated to API version **5.87**.

## [0.6.0] - 2018-09-14
### Changed
- Updated to API version **5.85**. See the [VK API Changelog](https://vk.com/dev/versions).

## [0.5.0] - 2018-07-19
### Added
- Objects! See [`rvk::objects (v0.5)`](https://docs.rs/rvk/0.5/rvk/objects/index.html).

## [0.4.0] - 2018-06-26
### Changed
- Type [`rvk::Params (v0.3.1)`](https://docs.rs/rvk/0.3.1/rvk/type.Params.html) was moved to [`rvk::api::Params (v0.4.0)`](https://docs.rs/rvk/0.4.0/rvk/api/type.Params.html), but it is now re-exported as `rvk::Params` too.

## [0.3.0] - 2018-06-19
### Changed
- This renames/adds some methods for VK API version **5.80** according to the [API Changelog](https://vk.com/dev/versions).

## [0.2.0] - 2018-05-31
### Changed
- API is now accessed synchronously
- Improved error handling

## [0.1.0] - 2018-05-26
### First release
