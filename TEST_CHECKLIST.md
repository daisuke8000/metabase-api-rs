# Release Infrastructure Test Checklist

## GitHub Actions Tests

### CI Pipeline
- [ ] Push to main → CI runs successfully
- [ ] Create PR → CI runs on PR
- [ ] Format check works
- [ ] Clippy check works
- [ ] Tests run successfully
- [ ] Security audit runs

### Release Drafter
- [ ] Create test PR with `feat:` prefix
- [ ] Create test PR with `fix:` prefix
- [ ] Merge PR → Draft release updates
- [ ] Release notes generated correctly
- [ ] Version resolver works

### Release Workflow
- [ ] Create manual release from draft
- [ ] Set tag (e.g., v0.1.0-test)
- [ ] Mark as pre-release
- [ ] Publish release
- [ ] Check if crates.io publish would trigger (dry-run)

## Testing Steps

1. **Create test branch**
   ```bash
   git checkout -b test/release-infra
   echo "test" >> test.txt
   git add test.txt
   git commit -m "feat: Test feature for release drafter"
   ```

2. **Create PR**
   - Push branch
   - Create PR with label `feat`
   - Check if CI runs

3. **Merge PR**
   - Merge to main
   - Check Release Drafter creates/updates draft

4. **Test Release**
   - Edit draft release
   - Add tag `v0.1.0-test`
   - Publish as pre-release
   - Verify workflows trigger

## Cleanup
- [ ] Delete test release
- [ ] Delete test tag
- [ ] Remove test commits (if needed)