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
   scp update-service.sh pi@your-pi-ip:~/deepl-proxy/
   scp setup-autostart.sh pi@your-pi-ip:~/deepl-proxy/
   scp deepl-proxy.service pi@your-pi-ip:~/deepl-proxy/
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

## Step 3: Setup Auto-Start (Optional but Recommended)

To automatically start the service on boot and enable automatic updates:

1. **Run the setup script:**
   ```bash
   chmod +x setup-autostart.sh
   sudo ./setup-autostart.sh
   ```

   This will:
   - Configure the service to start automatically on boot
   - Enable automatic version checking on startup
   - Pull the latest image from DockerHub before starting

2. **Test the service:**
   ```bash
   sudo systemctl start deepl-proxy
   sudo systemctl status deepl-proxy
   ```

## Step 4: Verify Deployment

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

## Automatic Updates

### Automatic Update on Boot
With the auto-start setup, the service will automatically:
- Check for new versions on DockerHub every time the Pi starts
- Pull the latest image if available
- Start with the updated version

### Manual Updates

1. **Using the update script (Recommended):**
   ```bash
   ./update-service.sh
   ```

2. **Manual update process:**
   ```bash
   docker-compose -f docker-compose.pi.yml pull
   docker-compose -f docker-compose.pi.yml up -d
   ```

## Legacy Update Method

For manual updates without the auto-start setup:

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

## Service Management Commands

Once auto-start is configured:

```bash
# Start service
sudo systemctl start deepl-proxy

# Stop service
sudo systemctl stop deepl-proxy

# Check status
sudo systemctl status deepl-proxy

# View logs
sudo journalctl -u deepl-proxy -f

# Restart service (will also check for updates)
sudo systemctl restart deepl-proxy
```

## Troubleshooting

- **Build fails:** Ensure Docker buildx supports ARM64: `docker buildx inspect --bootstrap`
- **Push fails:** Check Docker Hub credentials: `docker login`
- **Pi can't pull:** Verify network connectivity and image name
- **Service won't start:** Check logs: `docker-compose -f docker-compose.pi.yml logs`
- **Auto-update fails:** Check internet connection and DockerHub availability
- **Service fails to start:** Verify systemd service logs: `sudo journalctl -u deepl-proxy -f` 