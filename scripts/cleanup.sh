#!/bin/bash
echo "🧹 Cleaning up test containers..."

# Stop and remove containers
docker-compose down

# Remove test containers if they exist
docker rm -f warehouse_postgres_test warehouse_redis_test 2>/dev/null || true

# Remove unused images (optional)
# docker image prune -f

echo "✅ Cleanup complete"
