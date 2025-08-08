# Scripts Directory

This directory contains automation scripts for the metabase-api-rs project.

## Active Scripts

### setup-integration-env.sh
**Unified integration test environment setup script**

This is the main script for setting up the complete Metabase integration test environment with consistent test-specific values.

**Features:**
- Docker and Docker Compose prerequisite checks
- Automatic `.env.test` file creation
- Container lifecycle management
- PostgreSQL and Metabase health checks with configurable timeouts
- Metabase admin user creation
- Sample database connection setup
- Colored output for better readability
- Environment variable configuration support

**Usage:**
```bash
# Direct execution
./scripts/setup-integration-env.sh

# Via Taskfile (recommended)
task docker:up
task integration:run
```

**Environment Variables:**
- `METABASE_URL`: Metabase URL (default: http://localhost:3000)
- `METABASE_EMAIL`: Admin email (default: test-admin@metabase-test.local)
- `METABASE_PASSWORD`: Admin password (default: TestPassword123!)
- `METABASE_FIRST_NAME`: Admin first name (default: TestAdmin)
- `METABASE_LAST_NAME`: Admin last name (default: TestUser)
- `METABASE_SITE_NAME`: Site name (default: Test Environment Metabase)

### cleanup-docker.sh
**Complete Docker cleanup script**

Ensures thorough cleanup of all Docker resources created by the project.

**Features:**
- Stops and removes all project containers
- Removes both named and project-prefixed volumes
- Cleans up networks
- Removes dangling resources
- Verification of cleanup completion

**Usage:**
```bash
# Direct execution
./scripts/cleanup-docker.sh

# Via Taskfile (recommended)
task docker:down
task docker:clean
```

## Script Consolidation Benefits

The consolidation of the two setup scripts into `setup-integration-env.sh` provides:

1. **Single source of truth**: One script to maintain instead of two
2. **Consistent behavior**: Same setup process for all environments
3. **Better error handling**: Unified error handling and recovery
4. **Configuration flexibility**: Environment variable support for customization
5. **Improved maintainability**: Reduced code duplication
6. **Better user experience**: Consistent colored output and progress indicators

## Integration with Taskfile

All scripts are integrated with the project's Taskfile for convenient execution:

```bash
# Complete integration test workflow
task integration:run    # Full setup, test, and cleanup

# Docker management
task docker:up         # Setup environment
task docker:down       # Stop and cleanup
task docker:clean      # Complete cleanup
task docker:logs       # View logs

# Quick testing (assumes Docker is running)
task integration:quick
```