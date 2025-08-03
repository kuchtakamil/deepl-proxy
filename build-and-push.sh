#!/bin/bash

# Build and push script for cross-platform deployment
# Run this on your Ubuntu PC to build ARM64 image for Raspberry Pi

set -e

# Configuration - modify these variables as needed
DOCKER_USERNAME="${DOCKER_USERNAME:-kamilk86}"
IMAGE_NAME="${IMAGE_NAME:-deepl-proxy}"
TAG="${TAG:-latest}"
FULL_IMAGE_NAME="$DOCKER_USERNAME/$IMAGE_NAME:$TAG"

echo "🔧 Setting up Docker buildx for cross-platform builds..."

# Create and use buildx builder if it doesn't exist
if ! docker buildx ls | grep -q "multiarch"; then
    echo "Creating new buildx builder 'multiarch'..."
    docker buildx create --name multiarch --platform linux/amd64,linux/arm64 --use
fi

# Use the multiarch builder
docker buildx use multiarch

# Inspect builder to ensure ARM64 support
echo "📋 Available platforms:"
docker buildx inspect --bootstrap

echo "🏗️  Building ARM64 image: $FULL_IMAGE_NAME"

# Build and push the ARM64 image
docker buildx build \
    --platform linux/arm64 \
    --tag "$FULL_IMAGE_NAME" \
    --push \
    --file Dockerfile \
    .

echo "✅ Successfully built and pushed: $FULL_IMAGE_NAME"
echo ""
echo "📝 Next steps:"
echo "1. Copy docker-compose.pi.yml to your Raspberry Pi"
echo "2. Create .env file on Pi with your DEEPL_API_KEY"
echo "3. Run: docker-compose -f docker-compose.pi.yml pull"
echo "4. Run: docker-compose -f docker-compose.pi.yml up -d"
echo ""
echo "🔍 To check the image on Docker Hub:"
echo "   https://hub.docker.com/r/$DOCKER_USERNAME/$IMAGE_NAME" 