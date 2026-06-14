# OpenSteamController

This tool provides support for the Steam Controller (2026) for non Steam games, with full button support and status updates via built in tray app.
This is done by reading straight from the HID device itself and spawning a virtual controller per connected physical controller.

It provides sane defaults with buttons mapped like the Xbox controllers, all capacitive "buttons" disabled and paddles mapped to shoulder buttons.

## Features

- Connectivity via cable, puck or Bluetooth
- Multiple pucks simultaneously
- Mix and match connectivity styles
- Works on Linux and Windows
- Disables "Lizard Mode" (the  default mouse controls when not using steam)
- Status updates in tray app, this includes:
    - Available controller slots
    - Charging state
    - Battery level
    - Connectivity
- All digital and analog buttons, sticks and trackpads
- Support for all buttons, including:
    - All digital buttons, including paddles
    - Both sticks
    - Both trigger
    - Both trackpads

## Planned Features

- Switch between default and Nintendo layout (swap A/B X/Y)
- Provide custom, non-Xbox buttons for paddle/capacitive buttons mapping
- Read and send gyro/accelerometer events
- Toggle mouse/stick/dpad behaviour for trackpads
- Store controller configurations
- Provide AUR package
- Expand informations provided in tray
- Show battery status of lowest controller in tray icon
- Support rumble/haptics
- Shutdown via tool

### Maybe Features

- Support for MacOS
- Optionally "shadow" default steam controller

## Usage

### Requirements

#### Linux

- `uinput` kernel module

#### Windows

- ViGEm by Nefarius, <a href="https://docs.nefarius.at/projects/ViGEm/How-to-Install" title="nefarius ViGEm installation">install guide here</a>

### Installation

#### Recommended

Download the latest release and store it in an easy to reach place.
After that, just start it. The tool works, if the trackpads no longer control the mouse and/or a tray icon appears

## Screenshots

## Attribution
<a href="https://www.flaticon.com/free-icons/game-boy-advance" title="game boy advance icons">Game boy advance icons created by Freepik - Flaticon</a>
