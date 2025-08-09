# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [released]
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

[Unreleased]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-alpha.3...HEAD
[0.1.0-alpha.3]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-alpha.2...v0.1.0-alpha.3
[0.1.0-alpha.2]: https://github.com/daisuke8000/metabase-api-rs/compare/v0.1.0-alpha.1...v0.1.0-alpha.2
[0.1.0-alpha.1]: https://github.com/daisuke8000/metabase-api-rs/releases/tag/v0.1.0-alpha.1