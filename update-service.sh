#!/bin/bash

# Manual update script for DeepL Proxy on Raspberry Pi
# This script pulls the latest image and restarts the service

set -e

echo "🔄 Updating DeepL Proxy to latest version..."

# Check if we're in the right directory
if [ ! -f "docker-compose.pi.yml" ]; then
    echo "❌ Error: docker-compose.pi.yml not found in current directory"
    echo "Please run this script from the deepl_proxy directory"
    exit 1
fi

# Pull the latest image
echo "📥 Pulling latest image from DockerHub..."
docker-compose -f docker-compose.pi.yml pull

# Stop the current service
echo "🛑 Stopping current service..."
docker-compose -f docker-compose.pi.yml down

# Start with the new image
echo "🚀 Starting updated service..."
docker-compose -f docker-compose.pi.yml up -d

# Check status
echo "✅ Update complete! Checking service status..."
docker-compose -f docker-compose.pi.yml ps

echo ""
echo "🔍 To check if the service is running:"
echo "  curl http://localhost:3000/health"
echo ""
echo "📋 To view logs:"
echo "  docker-compose -f docker-compose.pi.yml logs -f deepl-proxy" 