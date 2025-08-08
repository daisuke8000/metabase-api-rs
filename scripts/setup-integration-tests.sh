#!/bin/bash
# Setup script for Metabase integration tests

set -e

echo "🚀 Setting up Metabase integration test environment..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Check if .env.test exists
if [ ! -f .env.test ]; then
    echo "📝 Creating .env.test file..."
    cat > .env.test << EOF
# Test environment configuration
METABASE_URL=http://localhost:3000
METABASE_TEST_EMAIL=admin@metabase.local
METABASE_TEST_PASSWORD=metabasepass123
RUST_LOG=debug
EOF
fi

# Stop any existing containers
echo "🛑 Stopping existing containers..."
docker-compose down 2>/dev/null || true

# Start the services
echo "🐳 Starting Docker containers..."
docker-compose up -d

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
for i in {1..30}; do
    if docker-compose exec -T postgres pg_isready -U metabase > /dev/null 2>&1; then
        echo "✅ PostgreSQL is ready!"
        break
    fi
    echo -n "."
    sleep 1
done

# Wait for Metabase to be ready
echo "⏳ Waiting for Metabase to be ready (this may take a minute)..."
for i in {1..60}; do
    if curl -s http://localhost:3000/api/health | grep -q "ok"; then
        echo "✅ Metabase is ready!"
        break
    fi
    echo -n "."
    sleep 2
done

# Setup Metabase initial configuration
echo "🔧 Setting up Metabase initial configuration..."

# Wait a bit more for Metabase to fully initialize
sleep 5

# Create admin user via API
echo "👤 Creating admin user..."
SETUP_TOKEN=$(curl -s http://localhost:3000/api/session/properties | grep -o '"setup-token":"[^"]*' | cut -d'"' -f4)

if [ ! -z "$SETUP_TOKEN" ]; then
    echo "Found setup token: $SETUP_TOKEN"
    
    # Perform initial setup
    curl -X POST http://localhost:3000/api/setup \
        -H "Content-Type: application/json" \
        -d "{
            \"token\": \"$SETUP_TOKEN\",
            \"user\": {
                \"email\": \"admin@metabase.local\",
                \"password\": \"metabasepass123\",
                \"first_name\": \"Admin\",
                \"last_name\": \"User\",
                \"site_name\": \"Test Metabase\"
            },
            \"database\": null,
            \"prefs\": {
                \"site_name\": \"Test Metabase\",
                \"site_locale\": \"en\",
                \"allow_tracking\": false
            }
        }" 2>/dev/null
    
    echo "✅ Admin user created!"
else
    echo "ℹ️  Metabase already configured (no setup token found)"
fi

# Add sample database connection
echo "🗄️ Adding sample database connection..."

# First, authenticate to get session token
SESSION_TOKEN=$(curl -s -X POST http://localhost:3000/api/session \
    -H "Content-Type: application/json" \
    -d '{"username":"admin@metabase.local","password":"metabasepass123"}' | \
    grep -o '"id":"[^"]*' | cut -d'"' -f4)

if [ ! -z "$SESSION_TOKEN" ]; then
    echo "Session token obtained"
    
    # Add the sample PostgreSQL database
    curl -X POST http://localhost:3000/api/database \
        -H "Content-Type: application/json" \
        -H "X-Metabase-Session: $SESSION_TOKEN" \
        -d '{
            "name": "Sample Database",
            "engine": "postgres",
            "details": {
                "host": "sample-postgres",
                "port": 5432,
                "dbname": "sampledb",
                "user": "sampleuser",
                "password": "samplepass",
                "ssl": false
            }
        }' 2>/dev/null
    
    echo "✅ Sample database added!"
fi

# Load sample data
echo "📊 Loading sample data into database..."
docker-compose exec -T sample-postgres psql -U sampleuser -d sampledb < tests/fixtures/sample_data.sql 2>/dev/null || true

echo ""
echo "✨ Integration test environment is ready!"
echo ""
echo "📌 Metabase URL: http://localhost:3000"
echo "📧 Admin Email: admin@metabase.local"
echo "🔑 Admin Password: metabasepass123"
echo ""
echo "Run integration tests with: task test:integration"
echo "Stop the environment with: docker-compose down"