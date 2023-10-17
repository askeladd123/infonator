#!/bin/bash

# Initialize variables
total_temp=0
count=0

# Loop through all thermal zones
for temp_input in /sys/class/thermal/thermal_zone*/temp; do
    # Read the temperature value (in milliCelsius)
    temp=$(cat $temp_input 2>/dev/null)
    
    # Check if the temperature value is valid
    if [ -n "$temp" ]; then
        # Convert to Celsius
        temp_c=$(echo "scale=1; $temp / 1000" | bc)
        
        # Accumulate total temperature
        total_temp=$(echo "$total_temp + $temp_c" | bc)
        
        # Increment count
        count=$((count + 1))
    fi
done

# Fallback to /sys/class/hwmon/hwmon2/temp1_input if no thermal zones found
if [ $count -eq 0 ]; then
    temp=$(cat /sys/class/hwmon/hwmon2/temp1_input 2>/dev/null)
    if [ -n "$temp" ]; then
        temp_c=$(echo "scale=1; $temp / 1000" | bc)
        total_temp=$temp_c
        count=1
    fi
fi

# Calculate average temperature
if [ $count -gt 0 ]; then
    avg_temp=$(echo "scale=1; $total_temp / $count" | bc)
    echo "${avg_temp}°C"
else
    echo "Could not read CPU temperature."
fi
