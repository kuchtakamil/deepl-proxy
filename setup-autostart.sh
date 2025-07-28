#!/bin/bash

# Setup script for DeepL Proxy auto-start on RaspberryPi boot
# Run this script as: sudo ./setup-autostart.sh

set -e

echo "🔧 Setting up DeepL Proxy auto-start on boot..."

# Get the current directory (where the script is located)
CURRENT_DIR=$(pwd)
SERVICE_USER=$(logname)
SERVICE_HOME="/home/$SERVICE_USER/deepl_proxy"

echo "📁 Application directory: $SERVICE_HOME"
echo "👤 Service user: $SERVICE_USER"

# Ensure Docker starts on boot
echo "🐳 Enabling Docker to start on boot..."
systemctl enable docker

# Update the service file with the correct paths
echo "📝 Updating service file with correct paths..."
sed -i "s|WorkingDirectory=.*|WorkingDirectory=$CURRENT_DIR|g" deepl-proxy.service
sed -i "s|ExecStart=.*|ExecStart=/usr/bin/docker-compose up -d|g" deepl-proxy.service
sed -i "s|ExecStop=.*|ExecStop=/usr/bin/docker-compose down|g" deepl-proxy.service

# Copy service file to systemd directory
echo "🔧 Installing systemd service..."
cp deepl-proxy.service /etc/systemd/system/

# Set proper permissions
chmod 644 /etc/systemd/system/deepl-proxy.service

# Reload systemd and enable the service
echo "🔄 Reloading systemd and enabling service..."
systemctl daemon-reload
systemctl enable deepl-proxy.service

echo "✅ Setup complete!"
echo ""
echo "📋 Service Management Commands:"
echo "  Start:   sudo systemctl start deepl-proxy"
echo "  Stop:    sudo systemctl stop deepl-proxy"
echo "  Status:  sudo systemctl status deepl-proxy"
echo "  Logs:    sudo journalctl -u deepl-proxy -f"
echo ""
echo "🔄 The service will now automatically start on boot."
echo "💡 To test: sudo systemctl start deepl-proxy"
echo ""
echo "⚠️  Make sure you have created the .env file with your DeepL API key!" 