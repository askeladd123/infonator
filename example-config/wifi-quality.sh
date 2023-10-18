#!/bin/bash

# Try to get the wireless interface name using iwconfig
interface=$(iwconfig 2>/dev/null | grep -o "^[[:alnum:]]*")

# Try to get WiFi quality using iwconfig
quality=$(iwconfig $interface 2>/dev/null | awk -F'[ =]+' '/Link Quality/ {print $3}')

# Fallback to iw if iwconfig fails
if [ -z "$quality" ]; then
    interface=$(iw dev | awk '$1=="Interface"{print $2}' 2>/dev/null)
    quality=$(iw dev $interface link 2>/dev/null | awk '/signal: / {print $2}')
    max_quality=-30  # Assuming -30 dBm is a good signal
fi

# Extract the current and max quality values
current_quality=$(echo $quality | cut -d'/' -f1)
max_quality=${max_quality:-$(echo $quality | cut -d'/' -f2)}

# Calculate quality percentage
if [ -n "$current_quality" ] && [ -n "$max_quality" ]; then
    quality_percentage=$(echo "scale=2; ($current_quality / $max_quality) * 100" | bc)
    echo "${quality_percentage}%"
else
    echo "Could not get WiFi quality."
fi
