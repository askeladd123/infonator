#!/bin/bash

# Get the backlight interface name (usually intel_backlight or acpi_video0)
backlight_interface=$(ls /sys/class/backlight/)

# Get the maximum brightness value
max_brightness=$(cat /sys/class/backlight/$backlight_interface/max_brightness)

# Get the current brightness value
current_brightness=$(cat /sys/class/backlight/$backlight_interface/brightness)

# Calculate brightness percentage
brightness_percentage=$(echo "scale=2; ($current_brightness / $max_brightness) * 100" | bc)

# Output the brightness percentage
echo "${brightness_percentage}%"
