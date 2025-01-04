# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.3] - 2025-01-04
- Update dependencies

## [0.2.2] - 2024-04-23
- Update dependencies

## [0.2.1] - 2024-01-29
- Update **tokio** to *1.35*
- Update **uuid** to *1.7*

## [0.2.0] - 2023-12-28
- Add an [axum-extractor](https://docs.rs/axum/latest/axum/#extractors) for extracting and validating Json in a request
- Improve some code comments
- Have Api `Err` responses return `(StatusCode, axum::Json<Value>)` instead of `(StatusCode, String)`
- Move Utility code for routes to *extractors.rs* and *response_builders.rs*
- Remove the `json_error()` response-builder, as it wasn't that usefull
- Add `clippy::unwrap_used` to clippy denies

## [0.1.3] - 2023-12-07
- Removed dev-dependency `hyper`, using `axum::http` instead
- Removed `cargo audit`, has not proved to be usefull
- Improve logging format settings

## [0.1.2] - 2023-12-06
- Fix integration-tests
- Disallow the usage of `.Unwrap()` in non-test code

## [0.1.1] - 2023-12-04
- Updated dependencies
- Updated Dockerfile

## [0.1.0] - 2023-06-25
- Added Tracing package for logging
- Updated dependencies
- Add validation using [Validator](https://docs.rs/validator)
