# Docker Deployment Guide for RaspberryPi

This guide will help you deploy the DeepL Proxy application on your RaspberryPi using Docker and Docker Compose.

## Prerequisites

1. **RaspberryPi** with Raspberry Pi OS (64-bit recommended)
2. **Docker** and **Docker Compose** installed
3. **DeepL API Key** from [DeepL API](https://www.deepl.com/pro-api)

## Installing Docker on RaspberryPi

If you haven't installed Docker yet:

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add your user to docker group
sudo usermod -aG docker $USER

# Install Docker Compose
sudo apt install docker-compose

# Restart to apply group changes
sudo reboot
```

## Deployment Steps

### 1. Clone and Setup

```bash
# Clone the repository (if not already done)
cd /home/pi  # or your preferred directory
git clone <your-repo-url> deepl_proxy
cd deepl_proxy
```

### 2. Configure Environment

Create a `.env` file with your DeepL API key:

```bash
# Create environment file
cat > .env << EOF
DEEPL_API_KEY=your-actual-deepl-api-key-here
RUST_LOG=info
EOF
```

**Important**: Replace `your-actual-deepl-api-key-here` with your real DeepL API key.

### 3. Build and Run

```bash
# Build and start the application
docker-compose up -d

# Check if it's running
docker-compose ps

# View logs
docker-compose logs -f deepl-proxy
```

### 4. Setup Auto-Start on Boot (Optional but Recommended)

To automatically start the application when your RaspberryPi boots:

```bash
# Make the setup script executable
chmod +x setup-autostart.sh

# Run the setup script (requires sudo)
sudo ./setup-autostart.sh
```

This will:
- Enable Docker to start on boot
- Create a systemd service for your application
- Configure automatic startup

**Manual systemd setup** (if you prefer manual setup):

```bash
# Enable Docker on boot
sudo systemctl enable docker

# Copy and enable the service
sudo cp deepl-proxy.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable deepl-proxy.service
```

### 5. Access the Application

Once running, you can access:
- **Web Application**: `http://your-pi-ip:3000` (complete application with UI)
- **API Endpoints**: 
  - `http://your-pi-ip:3000/translate` (POST - translate text)
  - `http://your-pi-ip:3000/improve` (POST - improve text)
  - `http://your-pi-ip:3000/health` (GET - health check)

**Finding your Pi's IP address:**
```bash
# On the Pi:
hostname -I

# Or try using hostname (may work without knowing IP):
http://raspberrypi.local:3000
```

**📡 For reliable access from other devices on your WiFi, see [`network-setup.md`](network-setup.md) for detailed network configuration options.**

## Management Commands

### Docker Compose Commands
```bash
# Stop the application
docker-compose down

# Update and restart
git pull
docker-compose down
docker-compose up -d --build

# View logs
docker-compose logs -f

# Check resource usage
docker stats deepl-proxy

# Remove everything (including volumes)
docker-compose down -v
```

### Systemd Service Commands (if auto-start is enabled)
```bash
# Start the service
sudo systemctl start deepl-proxy

# Stop the service
sudo systemctl stop deepl-proxy

# Check service status
sudo systemctl status deepl-proxy

# View service logs
sudo journalctl -u deepl-proxy -f

# Restart the service
sudo systemctl restart deepl-proxy

# Disable auto-start
sudo systemctl disable deepl-proxy

# Re-enable auto-start
sudo systemctl enable deepl-proxy
```

## Production Setup (Optional)

For production with SSL and nginx reverse proxy:

```bash
# Start with nginx proxy
docker-compose --profile production up -d
```

This requires setting up:
- `nginx.conf` configuration file
- SSL certificates in `./ssl/` directory

## Troubleshooting

### Build Issues
- **Out of memory**: The Pi might run out of memory during build. Add swap space:
  ```bash
  sudo dphys-swapfile swapoff
  sudo sed -i 's/CONF_SWAPSIZE=100/CONF_SWAPSIZE=1024/' /etc/dphys-swapfile
  sudo dphys-swapfile setup
  sudo dphys-swapfile swapon
  ```

### Runtime Issues
- **Container won't start**: Check logs with `docker-compose logs deepl-proxy` or `sudo journalctl -u deepl-proxy -f`
- **API errors**: Verify your DeepL API key in the `.env` file
- **Network issues**: Ensure port 3000 is not blocked by firewall
- **502/503 errors**: The application might still be starting, wait a few moments
- **Service fails to start on boot**: Check `sudo systemctl status deepl-proxy` and ensure Docker is running

### Performance Optimization
- The application is optimized for ARM64, but older Pis might need the ARM v7 variant
- For Pi 3 or older, change `linux/arm64` to `linux/arm/v7` in `docker-compose.yml`

## Monitoring

```bash
# Check resource usage
docker stats

# Check application health
curl http://localhost:3000/health

# Monitor logs in real-time
docker-compose logs -f --tail=100
```

## Backup

To backup your configuration:

```bash
# Backup environment and compose files
tar -czf deepl-proxy-backup.tar.gz .env docker-compose.yml

# Backup application data (if any)
docker-compose exec deepl-proxy tar -czf /tmp/app-data.tar.gz /app/data
docker cp deepl-proxy:/tmp/app-data.tar.gz ./app-data-backup.tar.gz
```

## Updates

### If using Docker Compose directly:
```bash
git pull
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

### If using systemd service:
```bash
# Stop the service
sudo systemctl stop deepl-proxy

# Update the application
git pull
docker-compose build --no-cache

# Start the service
sudo systemctl start deepl-proxy

# Check that it's running
sudo systemctl status deepl-proxy
```

## Security Notes

- Keep your DeepL API key secure and never commit it to version control
- Consider using Docker secrets for production deployments
- Regularly update the base images: `docker-compose pull && docker-compose up -d`
- Use a reverse proxy (nginx) with SSL for production deployments 