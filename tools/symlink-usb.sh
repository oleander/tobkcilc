#!/bin/bash

DEVICE_PATTERN="/dev/cu.*usbserial*"
SYMLINK="/dev/cu.wchusbserial140"

# Remove existing symlink
[ -L $SYMLINK ] && rm $SYMLINK

# Find device and create symlink
DEVICE=$(ls $DEVICE_PATTERN 2>/dev/null | head -n 1)
if [ ! -z "$DEVICE" ]; then
  ln -s $DEVICE $SYMLINK
  echo "Symlink created for $DEVICE"
else
  echo "Device not found"
fi
