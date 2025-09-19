#!/bin/bash

echo "🔍 Environment Status Check"
echo "=========================="

# Check Docker
if docker info &> /dev/null; then
    echo "✅ Docker is running"
else
    echo "❌ Docker is not running"
    echo "   Try: sudo systemctl start docker"
fi

# Check containers
if docker-compose ps | grep -q "Up"; then
    echo "✅ Database containers are running"
    docker-compose ps
else
    echo "❌ Database containers are not running"
    echo "   Try: docker-compose up -d"
fi

# Check ports
if ss -tuln | grep -q ":5432"; then
    echo "✅ PostgreSQL port 5432 is open"
else
    echo "❌ PostgreSQL port 5432 is not available"
fi

if ss -tuln | grep -q ":6379"; then
    echo "✅ Redis port 6379 is open"
else
    echo "❌ Redis port 6379 is not available"
fi

echo ""
echo "Development tools versions:"
echo "- Docker: $(docker --version | cut -d' ' -f3)"
echo "- Node.js: $(node --version)"
echo "- Rust: $(rustc --version | cut -d' ' -f2)"
echo "- Git: $(git --version | cut -d' ' -f3)"
