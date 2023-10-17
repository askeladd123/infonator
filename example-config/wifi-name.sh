#!/bin/bash

# Get the wireless interface name (usually wlan0, could be different)
interface=$(iw dev | awk '$1=="Interface"{print $2}')

# Get SSID using iw
ssid=$(iw dev $interface link | awk '/SSID/{print $2}')

# Output the SSID
echo $ssid
