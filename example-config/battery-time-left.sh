#!/bin/bash

# Get remaining battery time using acpi
battery_time=$(acpi -b | awk -F '[ ,]+' '/remaining/ {print $5}')

# Check if the battery is charging or discharging
charging_status=$(acpi -b | awk -F '[ ,]+' '{print $3}')

# Output the remaining battery time
if [ "$charging_status" == "Charging," ]; then
    echo "full in $battery_time"
else
    echo "empty in $battery_time"
fi
