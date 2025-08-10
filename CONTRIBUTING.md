# Contributing to metabase-api-rs

Thank you for your interest in contributing to metabase-api-rs!

## How to Contribute

### 1. Create an Issue

For bugs, feature requests, or questions:
- Search existing issues first
- Create a new issue with a clear title and description
- For bugs: include steps to reproduce and environment info
- For features: describe the use case and expected behavior

### 2. Fork and Create a Pull Request

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/metabase-api-rs.git
   cd metabase-api-rs
   ```
3. **Create a branch**:
   ```bash
   git checkout -b fix/issue-123
   # or
   git checkout -b feature/new-feature
   ```
4. **Make your changes** and add tests
5. **Run quality checks**:
   ```bash
   task check    # Format, lint, test
   ```
6. **Commit with conventional commit format**:
   ```bash
   git commit -m "fix: resolve authentication timeout issue"
   ```
7. **Push and create PR**:
   ```bash
   git push origin your-branch-name
   ```

### 3. Pull Request Review

- Link to related issues using `Closes #123`
- Describe what your changes do
- Wait for maintainer review
- Address feedback if needed

## Development Setup

**Prerequisites**: Rust 1.70+ and optionally [Task](https://taskfile.dev/)

```bash
# Clone and setup
git clone https://github.com/daisuke8000/metabase-api-rs.git
cd metabase-api-rs

# Development cycle
task dev     # Format, build, test
task check   # All quality checks
```

## Code Standards

- Use `cargo fmt` for formatting
- Fix all `cargo clippy` warnings
- Add tests for new functionality
- Follow existing code patterns
- Add documentation for public APIs

## Commit Messages

Use [Conventional Commits](https://conventionalcommits.org/):
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks

## License

By contributing, you agree your contributions will be licensed under MIT OR Apache-2.0.