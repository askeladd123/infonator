#!/bin/bash

# Get total and used memory using the free command
total_memory=$(free -m | awk '/Mem:/{print $2}')
used_memory=$(free -m | awk '/Mem:/{print $3}')

# Calculate RAM usage percentage
ram_percentage=$(echo "scale=2; ($used_memory / $total_memory) * 100" | bc)

# Output the RAM usage percentage
echo "${ram_percentage}%"
