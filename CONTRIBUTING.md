# Contributing to metabase-api-rs

Thank you for your interest in contributing to metabase-api-rs!

## Development Setup

1. Clone the repository
2. Install Rust (1.70+ required)
3. Run `task dev` to start development

## Development Workflow

Use Taskfile commands for development:
```bash
task dev    # Format, build, and test
task test   # Run all tests
task check  # Run quality checks
```

Tests are encouraged for all new features and bug fixes.

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Commit Convention

We use conventional commits:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `refactor:` Code refactoring
- `test:` Test additions
- `perf:` Performance improvements
- `chore:` Maintenance tasks

## Code Style

- Run `cargo fmt` before committing
- Check with `cargo clippy`
- Add tests for new functionality

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.