#!/bin/bash
# Get the current layout
current_layout=$(setxkbmap -query | grep layout | awk '{print $2}')

# Switch between layouts
if [ "$current_layout" = "us" ]; then
    setxkbmap cz  # Switch to Czech layout
    notify-send "Keyboard layout changed" "Layout: Czech (cz)"
else
    setxkbmap us  # Switch to US layout
    notify-send "Keyboard layout changed" "Layout: US (us)"
fi

