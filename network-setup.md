# Network Setup Guide for RaspberryPi Access

This guide covers different methods to ensure consistent access to your DeepL Proxy application from other devices on your home WiFi network.

## Method 1: Using Hostname (Easiest - Try This First!)

Most modern networks support mDNS, allowing you to access your Pi by hostname:

```bash
# From other devices on the same network, try:
http://raspberrypi.local:3000
```

**To find your Pi's hostname:**
```bash
# On the Pi, run:
hostname
```

**If your hostname is different (e.g., "mypi"), use:**
```bash
http://mypi.local:3000
```

**Pros:** No configuration needed, works automatically  
**Cons:** Might not work on all devices/networks

## Method 2: DHCP Reservation (Recommended)

Configure your router to always assign the same IP to your Pi based on its MAC address.

### Step 1: Find Your Pi's Information
```bash
# On the Pi, get the MAC address and current IP:
ip addr show wlan0 | grep "link/ether"
ip addr show wlan0 | grep "inet "

# Or use:
ifconfig wlan0
```

### Step 2: Router Configuration
1. **Access your router's admin panel** (usually `192.168.1.1` or `192.168.0.1`)
2. **Login** with admin credentials
3. **Find DHCP settings** (often under "Network" or "LAN Settings")
4. **Add DHCP Reservation:**
   - MAC Address: `xx:xx:xx:xx:xx:xx` (from Step 1)
   - IP Address: `192.168.1.100` (choose an available IP)
   - Description: `RaspberryPi-DeepL`
5. **Save and reboot** the router

### Step 3: Restart Pi Network
```bash
# On the Pi:
sudo systemctl restart networking
# Or reboot:
sudo reboot
```

**Pros:** Most reliable, router manages it  
**Cons:** Requires router access

## Method 3: Static IP on Pi (Alternative)

Configure a static IP directly on the RaspberryPi.

### Step 1: Check Current Network Settings
```bash
# Find your network info:
ip route | grep default
ip addr show wlan0
```

### Step 2: Edit Network Configuration
```bash
# Backup current config:
sudo cp /etc/dhcpcd.conf /etc/dhcpcd.conf.backup

# Edit the configuration:
sudo nano /etc/dhcpcd.conf
```

### Step 3: Add Static Configuration
Add these lines to the end of `/etc/dhcpcd.conf`:

```bash
# Static IP configuration for wlan0
interface wlan0
static ip_address=192.168.1.100/24
static routers=192.168.1.1
static domain_name_servers=192.168.1.1 8.8.8.8
```

**Important:** Adjust these values for your network:
- `192.168.1.100` - Choose an unused IP in your range
- `192.168.1.1` - Your router's IP (gateway)
- `192.168.1.1` - Your router's DNS (or use 8.8.8.8)

### Step 4: Apply Changes
```bash
# Restart networking:
sudo systemctl restart dhcpcd

# Or reboot:
sudo reboot

# Verify the new IP:
ip addr show wlan0
```

**Pros:** Controlled entirely from Pi  
**Cons:** Risk of IP conflicts, requires manual management

## Method 4: Finding Pi's Current IP

If you need to find your Pi's current IP address:

### From the Pi itself:
```bash
# Show all network interfaces:
ip addr show

# Show only WiFi IP:
hostname -I

# Show with more details:
ifconfig wlan0
```

### From another device on the network:
```bash
# Scan your network (Linux/Mac):
nmap -sn 192.168.1.0/24

# On Windows, use:
arp -a

# Or try:
ping raspberrypi.local
```

## Recommended Setup Steps

1. **Try Method 1 first** - Use `http://raspberrypi.local:3000`
2. **If that doesn't work**, use **Method 2** (DHCP Reservation)
3. **As last resort**, use **Method 3** (Static IP)

## Network Testing

Once configured, test access from other devices:

```bash
# Test connectivity:
ping 192.168.1.100  # (or your chosen IP)

# Test the application:
curl http://192.168.1.100:3000/health

# Or open in browser:
http://192.168.1.100:3000
```

## Firewall Considerations

If you can't access the application:

```bash
# On the Pi, check if UFW is blocking:
sudo ufw status

# If UFW is active, allow port 3000:
sudo ufw allow 3000

# For iptables:
sudo iptables -L

# Add rule if needed:
sudo iptables -A INPUT -p tcp --dport 3000 -j ACCEPT
```

## Router Port Forwarding (Optional - For Internet Access)

To access from outside your home network:

1. **Access router admin panel**
2. **Find Port Forwarding settings**
3. **Add rule:**
   - External Port: `8080` (or any available port)
   - Internal IP: `192.168.1.100` (your Pi's IP)
   - Internal Port: `3000`
   - Protocol: `TCP`
4. **Access via:** `http://your-public-ip:8080`

**Security Note:** Only do this if you understand the security implications and have proper authentication in place.

## Troubleshooting

### Can't access from other devices:
- Check Pi's firewall settings
- Verify both devices are on same network
- Try pinging the Pi first
- Check router's client isolation settings

### IP address keeps changing:
- DHCP reservation might not be configured correctly
- Try static IP configuration
- Check DHCP lease time on router

### "Connection refused" errors:
- Application might not be running: `sudo systemctl status deepl-proxy`
- Check if bound to correct interface: `netstat -tlnp | grep 3000`
- Verify Docker container is running: `docker ps`

## Quick Reference

| Method | Access URL | Reliability | Setup Difficulty |
|--------|-----------|-------------|------------------|
| Hostname | `http://raspberrypi.local:3000` | Good | None |
| DHCP Reservation | `http://192.168.1.100:3000` | Excellent | Medium |
| Static IP | `http://192.168.1.100:3000` | Good | Easy |

**Recommended:** Start with hostname, then move to DHCP reservation for production use. 