#!/bin/bash

# Network Helper Script for DeepL Proxy RaspberryPi Setup
# This script helps configure network settings for reliable access

set -e

echo "🌐 DeepL Proxy Network Configuration Helper"
echo "=============================================="
echo

# Color codes for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if running as root for network changes
check_root() {
    if [[ $EUID -eq 0 ]]; then
        print_warning "Running as root - can make network changes"
        return 0
    else
        print_warning "Not running as root - can only display information"
        return 1
    fi
}

# Get current network information
get_network_info() {
    echo "📋 Current Network Information:"
    echo "------------------------------"
    
    # Hostname
    HOSTNAME=$(hostname)
    echo "Hostname: $HOSTNAME"
    
    # WiFi interface info
    if ip link show wlan0 &> /dev/null; then
        # IP Address
        WIFI_IP=$(ip addr show wlan0 | grep "inet " | awk '{print $2}' | cut -d/ -f1)
        echo "WiFi IP: ${WIFI_IP:-Not assigned}"
        
        # MAC Address
        WIFI_MAC=$(ip addr show wlan0 | grep "link/ether" | awk '{print $2}')
        echo "WiFi MAC: $WIFI_MAC"
        
        # Gateway
        GATEWAY=$(ip route | grep default | awk '{print $3}' | head -1)
        echo "Gateway: ${GATEWAY:-Not found}"
        
        # DNS
        DNS=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2}' | head -1)
        echo "DNS: ${DNS:-Not configured}"
        
        echo
        print_status "Access URLs:"
        if [ -n "$WIFI_IP" ]; then
            echo "  Direct IP: http://$WIFI_IP:3000"
        fi
        echo "  Hostname:  http://$HOSTNAME.local:3000"
        
    else
        print_error "WiFi interface (wlan0) not found"
        return 1
    fi
}

# Test network connectivity
test_connectivity() {
    echo
    echo "🔍 Testing Network Connectivity:"
    echo "--------------------------------"
    
    # Test internet connectivity
    if ping -c 1 8.8.8.8 &> /dev/null; then
        print_status "Internet connectivity: OK"
    else
        print_error "Internet connectivity: FAILED"
    fi
    
    # Test gateway connectivity
    if [ -n "$GATEWAY" ] && ping -c 1 "$GATEWAY" &> /dev/null; then
        print_status "Gateway connectivity: OK"
    else
        print_error "Gateway connectivity: FAILED"
    fi
    
    # Test if port 3000 is open (if application is running)
    if netstat -tlnp 2>/dev/null | grep -q ":3000 "; then
        print_status "Application port 3000: LISTENING"
    else
        print_warning "Application port 3000: NOT LISTENING (app may not be running)"
    fi
}

# Suggest network configuration
suggest_config() {
    echo
    echo "💡 Recommended Network Setup:"
    echo "-----------------------------"
    
    if [ -n "$WIFI_IP" ]; then
        # Calculate suggested static IP (add 50 to current IP)
        IFS='.' read -ra IP_PARTS <<< "$WIFI_IP"
        SUGGESTED_IP="${IP_PARTS[0]}.${IP_PARTS[1]}.${IP_PARTS[2]}.$((${IP_PARTS[3]} + 50))"
        
        echo "Current IP: $WIFI_IP (likely dynamic)"
        echo "Suggested static IP: $SUGGESTED_IP"
        echo
        echo "Configuration options:"
        echo "1. Easy: Use hostname access (http://$HOSTNAME.local:3000)"
        echo "2. Best: Configure DHCP reservation on your router"
        echo "3. Alternative: Set static IP on Pi"
        echo
        
        if check_root; then
            echo "📝 Would you like to configure a static IP? (y/N): "
            read -r response
            if [[ "$response" =~ ^[Yy]$ ]]; then
                configure_static_ip "$SUGGESTED_IP"
            fi
        else
            echo "Run with sudo to configure static IP: sudo $0"
        fi
    fi
}

# Configure static IP
configure_static_ip() {
    local static_ip="$1"
    
    echo
    echo "🔧 Configuring Static IP: $static_ip"
    echo "-----------------------------------"
    
    # Backup current configuration
    if [ -f /etc/dhcpcd.conf ]; then
        cp /etc/dhcpcd.conf /etc/dhcpcd.conf.backup.$(date +%Y%m%d_%H%M%S)
        print_status "Backed up current configuration"
    fi
    
    # Check if static config already exists
    if grep -q "interface wlan0" /etc/dhcpcd.conf; then
        print_warning "Static configuration already exists in /etc/dhcpcd.conf"
        echo "Please check the file manually or remove existing configuration first"
        return 1
    fi
    
    # Add static configuration
    cat >> /etc/dhcpcd.conf << EOF

# Static IP configuration for DeepL Proxy (added $(date))
interface wlan0
static ip_address=$static_ip/24
static routers=$GATEWAY
static domain_name_servers=$GATEWAY 8.8.8.8
EOF
    
    print_status "Static IP configuration added to /etc/dhcpcd.conf"
    echo
    echo "⚠️  Changes will take effect after reboot or network restart"
    echo "   To apply now: sudo systemctl restart dhcpcd"
    echo "   To reboot: sudo reboot"
    echo
    echo "   New access URL will be: http://$static_ip:3000"
}

# Show firewall status
check_firewall() {
    echo
    echo "🔥 Firewall Status:"
    echo "------------------"
    
    # Check UFW
    if command -v ufw &> /dev/null; then
        UFW_STATUS=$(ufw status | head -1)
        echo "UFW: $UFW_STATUS"
        
        if [[ "$UFW_STATUS" == *"active"* ]]; then
            if ufw status | grep -q "3000"; then
                print_status "Port 3000 is allowed in UFW"
            else
                print_warning "Port 3000 not explicitly allowed in UFW"
                if check_root; then
                    echo "Add UFW rule? (y/N): "
                    read -r response
                    if [[ "$response" =~ ^[Yy]$ ]]; then
                        ufw allow 3000
                        print_status "Added UFW rule for port 3000"
                    fi
                fi
            fi
        fi
    else
        echo "UFW: Not installed"
    fi
    
    # Check iptables
    if iptables -L INPUT 2>/dev/null | grep -q "dpt:3000"; then
        print_status "Port 3000 allowed in iptables"
    elif iptables -L INPUT 2>/dev/null | grep -q "REJECT\|DROP"; then
        print_warning "iptables may be blocking port 3000"
    fi
}

# Main execution
main() {
    get_network_info
    test_connectivity
    check_firewall
    suggest_config
    
    echo
    echo "📚 For more detailed network setup options, see: network-setup.md"
    echo "🐳 To check application status: sudo systemctl status deepl-proxy"
}

# Help function
show_help() {
    echo "DeepL Proxy Network Helper"
    echo
    echo "Usage: $0 [options]"
    echo
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  --info-only    Only show network information"
    echo "  --static-ip IP Configure static IP address"
    echo
    echo "Examples:"
    echo "  $0                           # Full network check and setup"
    echo "  $0 --info-only              # Just show current network info"
    echo "  sudo $0 --static-ip 192.168.1.100  # Set specific static IP"
}

# Parse command line arguments
case "${1:-}" in
    -h|--help)
        show_help
        exit 0
        ;;
    --info-only)
        get_network_info
        exit 0
        ;;
    --static-ip)
        if [ -z "$2" ]; then
            print_error "Please provide an IP address"
            exit 1
        fi
        if ! check_root; then
            print_error "Root privileges required for network configuration"
            exit 1
        fi
        get_network_info
        configure_static_ip "$2"
        exit 0
        ;;
    "")
        main
        ;;
    *)
        print_error "Unknown option: $1"
        show_help
        exit 1
        ;;
esac 