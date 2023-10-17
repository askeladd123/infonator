#!/bin/bash

# Try to get the volume level using amixer
volume_level=$(amixer get Master 2>/dev/null | awk -F'[][]' '/%/ {print $2}' | head -n 1)

# Fallback to pactl if amixer fails
if [ -z "$volume_level" ]; then
    volume_level=$(pactl get-sink-volume @DEFAULT_SINK@ 2>/dev/null | awk '/Volume:/ {print $5}' | sed 's/%//')
    volume_level="${volume_level}%"
fi

# Check if we got the volume level
if [ -n "$volume_level" ]; then
    echo "$volume_level"
else
    echo "Could not get volume level."
fi
