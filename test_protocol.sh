#!/bin/bash

# Test script to simulate Minecraft client packets
# This sends the same packets that were captured in the logs

echo "Testing Minecraft protocol handling..."

# Send handshake + status request packets using netcat
echo -ne '\x10\x00\x82\x06\x09localhost\x63\xdd\x01\x01\x00' | nc localhost 25565

echo "Test completed."
