# Cross-Platform Deployment Guide

This guide explains how to build the Docker image on your Ubuntu PC and deploy it to your Raspberry Pi.

## Prerequisites

### On Ubuntu PC:
- Docker installed with buildx support
- Docker Hub account (or access to another container registry)

### On Raspberry Pi:
- Docker installed
- Docker Compose installed
- Network access to pull images

## Step 1: Setup on Ubuntu PC

1. **Login to Docker Hub:**
   ```bash
   docker login
   ```

2. **Configure the build script:**
   Edit `build-and-push.sh` and update these variables:
   ```bash
   DOCKER_USERNAME="your-dockerhub-username"
   IMAGE_NAME="deepl-proxy"
   TAG="latest"
   ```

3. **Make the script executable:**
   ```bash
   chmod +x build-and-push.sh
   ```

4. **Build and push the image:**
   ```bash
   ./build-and-push.sh
   ```

   This will:
   - Set up Docker buildx for cross-platform builds
   - Build the ARM64 image for Raspberry Pi
   - Push it to Docker Hub

## Step 2: Deploy on Raspberry Pi

1. **Copy deployment files to your Pi:**
   ```bash
   scp docker-compose.pi.yml pi@your-pi-ip:~/deepl-proxy/
   scp .env pi@your-pi-ip:~/deepl-proxy/  # If you have environment variables
   ```

2. **SSH into your Raspberry Pi:**
   ```bash
   ssh pi@your-pi-ip
   cd ~/deepl-proxy
   ```

3. **Update the image name in docker-compose.pi.yml:**
   Edit `docker-compose.pi.yml` and replace `your-dockerhub-username` with your actual Docker Hub username.

4. **Create .env file (if needed):**
   ```bash
   echo "DEEPL_API_KEY=your-actual-api-key" > .env
   ```

5. **Pull and run the application:**
   ```bash
   # Pull the pre-built image
   docker-compose -f docker-compose.pi.yml pull
   
   # Start the application
   docker-compose -f docker-compose.pi.yml up -d
   
   # Check status
   docker-compose -f docker-compose.pi.yml ps
   ```

## Step 3: Verify Deployment

1. **Check if the service is running:**
   ```bash
   curl http://localhost:3000/health
   ```

2. **View logs if needed:**
   ```bash
   docker-compose -f docker-compose.pi.yml logs -f deepl-proxy
   ```

3. **Access the web interface:**
   Open `http://your-pi-ip:3000` in your browser

## Updating the Application

When you make changes:

1. **On Ubuntu PC:**
   ```bash
   ./build-and-push.sh
   ```

2. **On Raspberry Pi:**
   ```bash
   docker-compose -f docker-compose.pi.yml pull
   docker-compose -f docker-compose.pi.yml up -d
   ```

## Alternative Registries

If you prefer not to use Docker Hub, you can use:

- **GitHub Container Registry:** 
  - Change `DOCKER_USERNAME` to `ghcr.io/your-github-username`
  - Login with: `echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin`

- **Private Registry:**
  - Update the image name in both scripts accordingly
  - Ensure your Pi can access the private registry

## Troubleshooting

- **Build fails:** Ensure Docker buildx supports ARM64: `docker buildx inspect --bootstrap`
- **Push fails:** Check Docker Hub credentials: `docker login`
- **Pi can't pull:** Verify network connectivity and image name
- **Service won't start:** Check logs: `docker-compose -f docker-compose.pi.yml logs` 