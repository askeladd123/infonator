#!/bin/bash

# Get the battery name (usually BAT0 or BAT1)
battery_name=$(ls /sys/class/power_supply/ | grep 'BAT')

# Get battery capacity
battery_percentage=$(cat /sys/class/power_supply/$battery_name/capacity)

# Output the battery percentage
echo "$battery_percentage%"
