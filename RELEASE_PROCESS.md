# Release Process

## Quick Release Steps

### For Alpha/Beta releases:
```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md
# 3. Commit and tag
git add -A
git commit -m "chore: Release vX.Y.Z"
git tag -a vX.Y.Z -m "Release vX.Y.Z"

# 4. Push to GitHub
git push origin main --tags

# 5. Publish to crates.io
cargo publish
```

## Version Format
- Alpha: `0.1.0-alpha.N`
- Beta: `0.1.0-beta.N`
- Release: `0.1.0`