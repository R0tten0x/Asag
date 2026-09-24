# Asag

macOS `launchd` background service that automatically manages Wi-Fi based on an active Ethernet connection.

## What It Does

- Detects an active Ethernet connection.
- Disables Wi-Fi when Ethernet is connected.
- Re-enables Wi-Fi when Ethernet is disconnected.
- Sends a macOS notification when the connection state changes.
- Only works when laptop is open. Clamshell mode is not supported. (Manually re-enabling wifi needed)

## Why

MacBooks don't have built-in Ethernet ports. Ethernet is typically connected through a USB-C or Thunderbolt adapter/dock.

I frequently move between my desk and other locations. Manually disconnecting Wi-Fi when Ethernet is connected, then re-enabling it when Ethernet is disconnected, got annoying.

So I wrote this.

## Installation

### Option 1 — Clone and Build

    git clone https://github.com/R0tten0x/Asag.git
    cd Asag
    cargo build --release

Copy the binary to the preferred location:

    sudo mkdir -p /opt/apps
    sudo cp target/release/asag /opt/apps/asag

Copy the launchd plist:

    cp dev.rotten0x.asag.plist ~/Library/LaunchAgents/

Load the service:

    launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/dev.rotten0x.asag.plist

### Option 2 — Release Binary

Download the release binary and `dev.rotten0x.asag.plist`.

Copy the binary to:

    /opt/apps/asag

Copy the plist to:

    ~/Library/LaunchAgents/dev.rotten0x.asag.plist

Then load the service:

    launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/dev.rotten0x.asag.plist

## Requirements

- macOS
- Ethernet connection through USB-C, Thunderbolt, or a dock
- Wi-Fi interface `en0`

## Uninstall

Unload the service:

    launchctl bootout gui/$(id -u)/dev.rotten0x.asag

Remove the plist:

    rm ~/Library/LaunchAgents/dev.rotten0x.asag.plist

Remove the binary:

    sudo rm /opt/apps/asag

## License
Edit to your liking, give credit where it's due. 
