#!/bin/bash
# Unified setup script for Metabase integration test environment

set -e

# ========== Configuration ==========
METABASE_URL="${METABASE_URL:-http://localhost:3000}"
METABASE_EMAIL="${METABASE_EMAIL:-test-admin@metabase-test.local}"
METABASE_PASSWORD="${METABASE_PASSWORD:-TestPassword123!}"
METABASE_FIRST_NAME="${METABASE_FIRST_NAME:-TestAdmin}"
METABASE_LAST_NAME="${METABASE_LAST_NAME:-TestUser}"
METABASE_SITE_NAME="${METABASE_SITE_NAME:-Test Environment Metabase}"

# Timeouts
POSTGRES_TIMEOUT=60
METABASE_TIMEOUT=300
METABASE_HEALTH_CHECK_INTERVAL=5

# ========== Color Output Functions ==========
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}ℹ️${NC}  $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC}  $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

# ========== Prerequisite Checks ==========
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check Docker
    if ! docker info >/dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker first."
        exit 1
    fi
    
    # Check Docker Compose
    if ! command -v docker-compose >/dev/null 2>&1; then
        print_error "docker-compose is not installed or not in PATH."
        exit 1
    fi
    
    # Check/Create .env.test if needed
    if [ ! -f .env.test ]; then
        print_status "Creating .env.test file..."
        cat > .env.test << EOF
# Test environment configuration
METABASE_URL=${METABASE_URL}
METABASE_EMAIL=${METABASE_EMAIL}
METABASE_PASSWORD=${METABASE_PASSWORD}
METABASE_API_KEY=test_api_key_development_only_12345

# Sample database connection for testing
SAMPLE_DB_HOST=localhost
SAMPLE_DB_PORT=5433
SAMPLE_DB_NAME=test_sample_db
SAMPLE_DB_USER=test_sample_user
SAMPLE_DB_PASSWORD=TestSamplePassword456!

# Test configuration
TEST_TIMEOUT_SECONDS=30
TEST_RETRY_COUNT=3
TEST_LOG_LEVEL=debug
EOF
        print_success ".env.test created"
    fi
}

# ========== Docker Management ==========
stop_existing_containers() {
    print_status "Stopping existing containers..."
    docker-compose down --remove-orphans 2>/dev/null || true
}

start_containers() {
    print_status "Starting Docker containers..."
    docker-compose up -d
    
    if [ $? -ne 0 ]; then
        print_error "Failed to start Docker containers"
        exit 1
    fi
}

# ========== Service Health Checks ==========
wait_for_postgres() {
    print_status "Waiting for PostgreSQL to be ready..."
    
    local elapsed=0
    while [ $elapsed -lt $POSTGRES_TIMEOUT ]; do
        if docker exec metabase-postgres pg_isready -U test_metabase_user >/dev/null 2>&1; then
            print_success "PostgreSQL is ready!"
            return 0
        fi
        echo -n "."
        sleep 2
        elapsed=$((elapsed + 2))
    done
    
    print_error "PostgreSQL failed to start within ${POSTGRES_TIMEOUT} seconds"
    return 1
}

wait_for_metabase() {
    print_status "Waiting for Metabase to be ready (this may take a few minutes)..."
    
    local elapsed=0
    while [ $elapsed -lt $METABASE_TIMEOUT ]; do
        if curl -s "${METABASE_URL}/api/health" | grep -q "ok" >/dev/null 2>&1; then
            print_success "Metabase is ready!"
            return 0
        fi
        echo -n "."
        sleep $METABASE_HEALTH_CHECK_INTERVAL
        elapsed=$((elapsed + METABASE_HEALTH_CHECK_INTERVAL))
    done
    
    print_error "Metabase failed to start within ${METABASE_TIMEOUT} seconds"
    print_status "Checking container logs..."
    docker-compose logs metabase | tail -50
    return 1
}

# ========== Metabase Setup ==========
setup_metabase_admin() {
    print_status "Setting up Metabase admin user..."
    
    # Wait a bit for full initialization
    sleep 5
    
    # Get setup token
    local setup_token=$(curl -s "${METABASE_URL}/api/session/properties" | \
        grep -o '"setup-token":"[^"]*' | cut -d'"' -f4)
    
    if [ -z "$setup_token" ]; then
        print_warning "Metabase already configured (no setup token found)"
        return 0
    fi
    
    print_status "Found setup token, creating admin user..."
    
    # Perform initial setup
    local setup_response=$(curl -s -X POST "${METABASE_URL}/api/setup" \
        -H "Content-Type: application/json" \
        -d "{
            \"token\": \"$setup_token\",
            \"user\": {
                \"email\": \"${METABASE_EMAIL}\",
                \"password\": \"${METABASE_PASSWORD}\",
                \"first_name\": \"${METABASE_FIRST_NAME}\",
                \"last_name\": \"${METABASE_LAST_NAME}\"
            },
            \"database\": null,
            \"prefs\": {
                \"site_name\": \"${METABASE_SITE_NAME}\",
                \"site_locale\": \"en\",
                \"allow_tracking\": false
            }
        }")
    
    if echo "$setup_response" | grep -q '"id"' >/dev/null 2>&1; then
        print_success "Admin user created successfully"
        return 0
    else
        print_warning "Setup response: $setup_response"
        return 1
    fi
}

add_sample_database() {
    print_status "Adding sample database connection..."
    
    # Authenticate first
    local session_response=$(curl -s -X POST "${METABASE_URL}/api/session" \
        -H "Content-Type: application/json" \
        -d "{\"username\":\"${METABASE_EMAIL}\",\"password\":\"${METABASE_PASSWORD}\"}")
    
    local session_id=$(echo "$session_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4)
    
    if [ -z "$session_id" ]; then
        print_warning "Failed to authenticate for database setup"
        return 1
    fi
    
    print_status "Session obtained, adding sample database..."
    
    # Add sample database
    local db_response=$(curl -s -X POST "${METABASE_URL}/api/database" \
        -H "Content-Type: application/json" \
        -H "X-Metabase-Session: $session_id" \
        -d '{
            "name": "Test Sample Database",
            "engine": "postgres",
            "details": {
                "host": "sample-postgres",
                "port": 5432,
                "dbname": "test_sample_db",
                "user": "test_sample_user",
                "password": "TestSamplePassword456!",
                "ssl": false
            }
        }')
    
    if echo "$db_response" | grep -q '"id"' >/dev/null 2>&1; then
        print_success "Sample database added successfully"
        return 0
    else
        print_warning "Database response: $db_response"
        return 1
    fi
}

# ========== Main Execution ==========
main() {
    echo "🚀 Setting up Metabase integration test environment..."
    echo ""
    
    # Run setup steps
    check_prerequisites
    stop_existing_containers
    start_containers
    
    # Wait for services
    wait_for_postgres || exit 1
    wait_for_metabase || exit 1
    
    # Configure Metabase
    setup_metabase_admin
    add_sample_database
    
    # Print summary
    echo ""
    echo "✨ Integration test environment is ready!"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📌 Test Environment Details:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Metabase URL:     ${METABASE_URL}"
    echo "  Admin Email:      ${METABASE_EMAIL}"
    echo "  Admin Password:   ${METABASE_PASSWORD}"
    echo "  PostgreSQL:       localhost:5432 (Test Metabase DB)"
    echo "  Sample DB:        localhost:5433 (Test Sample DB)"
    echo ""
    echo "📝 Common Commands:"
    echo "  Run tests:        task test:integration"
    echo "  View logs:        docker-compose logs -f"
    echo "  Stop services:    task docker:down"
    echo "  Clean all:        task docker:clean"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Run main function
main "$@"