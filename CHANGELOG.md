# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0-beta.3] - 2025-08-10

### Fixed
- README.md alpha to beta

## [0.1.0-beta.2] - 2025-08-10

### Fixed
- Fixed critical bug in `execute_raw_query` method that was returning empty results
- Query execution now properly parses HTTP response data and returns actual query results
- Fixed `execute_dataset_query` functionality in architecture tests

### Changed
- Improved response parsing in query repository layer
- Enhanced error handling for malformed API responses
- Code optimization to comply with Clippy lints (removed redundant closures and improved iterator usage)

## [0.1.0-beta.1] - 2025-08-10

### Fixed
- Fixed authentication API usage in benchmarks and examples
- Corrected `Credentials::EmailPassword` to `Credentials::email_password()` method
- Fixed `MetabaseId` import path in crud_simple.rs example

### Changed

- Updated README.md security statement to reflect resolved vulnerabilities
- Improved architecture test coverage with consolidated test file
- Enhanced performance benchmarking with realistic scenarios


## [0.1.0-alpha.3] - 2025-08-09

### Changed
- Simplified README.md significantly (48% reduction)
- Removed Architecture and Development sections from README
- Updated documentation to use table format for features
- Fixed docs.rs links to be version-agnostic

## [0.1.0-alpha.2] - 2025-08-09

### Fixed
- Added all example files to Cargo.toml for docs.rs visibility
- Fixed examples not appearing in documentation

## [0.1.0-alpha.1] - 2025-08-09

### Added
- Initial alpha release
- Core API client with automatic authentication
- Session management with auto-refresh
- Retry logic with exponential backoff
- Card CRUD operations
- Collection CRUD operations
- Dashboard CRUD operations
- Database operations
- Query execution (SQL and MBQL)
- User management
- Permission management
- Alert management
- Pulse management
- Segment and metric operations
- Cache layer with LRU support
- Comprehensive test suite (100+ tests)
- Security hardening features
- Example code and documentation

### Security
- Secure credential handling with zeroize
- Protected session tokens
- Input validation and sanitization
- TLS/HTTPS enforcement for production

### Known Issues
- Integration tests require actual Metabase instance
- Some advanced MBQL features not yet implemented
- Performance benchmarks are synthetic

[released]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-beta.3...HEAD
[0.1.0-beta.3]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-beta.2...v0.1.0-beta.3
[0.1.0-beta.2]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-beta.1...v0.1.0-beta.2
[0.1.0-beta.1]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-alpha.3...v0.1.0-beta.1
[0.1.0-alpha.3]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-alpha.2...v0.1.0-alpha.3
[0.1.0-alpha.2]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-alpha.1...v0.1.0-alpha.2
[0.1.0-alpha.1]: https://github.com/daisuke8000/metabase-api-rs/releases/tag/v0.1.0-alpha.1