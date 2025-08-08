#!/bin/bash
# Complete Docker cleanup script for metabase-api-rs project

set -e

echo "🧹 Starting complete Docker cleanup..."

PROJECT_NAME="metabase-api-rs"

# Stop all containers related to this project
echo "📦 Stopping containers..."
docker-compose down 2>/dev/null || true
docker stop metabase-app metabase-postgres sample-database 2>/dev/null || true

# Remove containers
echo "🗑️  Removing containers..."
docker rm -f metabase-app metabase-postgres sample-database 2>/dev/null || true

# Remove volumes (both named and project-prefixed)
echo "💾 Removing volumes..."
docker volume rm postgres_data 2>/dev/null || true
docker volume rm ${PROJECT_NAME}_postgres_data 2>/dev/null || true
docker volume ls | grep -E "(metabase|sample)" | awk '{print $2}' | xargs -r docker volume rm 2>/dev/null || true

# Remove networks
echo "🌐 Removing networks..."
docker network rm metabase-network 2>/dev/null || true
docker network rm ${PROJECT_NAME}_metabase-network 2>/dev/null || true
docker network ls | grep metabase | awk '{print $2}' | xargs -r docker network rm 2>/dev/null || true

# Clean up dangling resources
echo "🧽 Cleaning up dangling resources..."
docker system prune -f --volumes 2>/dev/null || true

# Verify cleanup
echo ""
echo "📊 Verification:"
echo "  Containers:"
docker ps -a | grep -E "(metabase|sample)" || echo "    ✅ No related containers found"
echo "  Volumes:"
docker volume ls | grep -E "(metabase|sample|postgres_data)" || echo "    ✅ No related volumes found"
echo "  Networks:"
docker network ls | grep metabase || echo "    ✅ No related networks found"

echo ""
echo "✨ Docker cleanup complete!"