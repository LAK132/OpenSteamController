# OpenSteamController

This tool provides support for the Steam Controller (2026) for non Steam games, with full button support and status updates via built in tray app.
This is done by reading straight from the HID device itself and spawning a virtual controller per connected physical controller.

It provides sane defaults with buttons mapped like the Xbox controllers, all capacitive "buttons" disabled and paddles mapped to shoulder buttons.

This project is not affiliated with, endorsed by, or associated with Valve in any way. All trademarks and brand names belong to their respective owners.

## Screenshots

| OS | Tooltip |
|:---:|:---:|
| **Linux** | <img src=./screenshots/linux-tray.png width="280"> |
| **Windows** | <img src=./screenshots/windows-tray.png width="280"> |

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
- Shutdown controller via tray or (Steam button + Y)
- Switch between default and Nintendo layout (swap A/B X/Y) via tray or (Steam button + A)
- Trackpad mouse (Linux only)


## Planned Feature

- Provide custom, non-Xbox buttons for paddle/capacitive buttons mapping
- Read and send gyro/accelerometer events
- Toggle mouse/stick/dpad behaviour for trackpads
- Store controller configurations
- Provide AUR package
- Publish on crates.io
- Expand informations provided in tray
- Show battery status of lowest controller in tray icon
- Support rumble/haptics

#### Maybe Features

- Support for MacOS
- Optionally "shadow" default steam controller

## Usage

```
open-steam-controller --help
A tray application providing extended support for the Steam Controller (2026) without Steam.

Usage: open-steam-controller [OPTIONS]

Options:
  -v, --verbose               Use verbose output
      --monochrome-icons      Use the symbolic (monochrome) variants of the system tray icons
  -e, --emulate <CONTROLLER>  Which device should the virtual controller emulate
  -h, --help                  Print help
  -V, --version               Print version
```

| Emulated controller | `<CONTROLLER>` | Windows | Linux | D-pad hat | D-pad buttons | Trigger buttons | Paddles |
|:---|:---|:---:|:---:|:---:|:---:|:---:|:---:|
| Default | | Xbox 360 | Xbox One Elite |
| Xbox 360 | `x360` or `xbox360` | &check; | &check; | &check; | &cross; | &cross; | &cross; |
| Xbox One Elite | `xbe` or `xboxelite` | &cross; | &check; | &check; | &check; | &check; | &check; |
| DualShock 4 | `ds4` or `dualshock4` | &cross; | &check; | &check; | &check; | &check; | &cross; |
| DualSense Edge | `dse` or `dualsenseedge` | &cross; | &check; | &check; | &check; | &check; | &check; |

| Steam Controller | Xbox layout | Nintendo layout | Lizard layout |
|:---|:---|:---|:---|
| A | A | B | Enter |
| B | B | A | Esc |
| X | X | Y | Backspace |
| Y | Y | X | |
| Dpad up | Dpad up | Dpad up | Arrow up |
| Dpad down | Dpad down | Dpad down | Arrow down |
| Dpad left | Dpad left | Dpad left | Arrow left |
| Dpad right | Dpad right | Dpad right | Arrow right |
| Start/Menu | Menu | Start/+ | Meta/Win |
| Select/View | View | Select/- | Tab |
| L stick | L stick | L stick | Smooth scroll |
| R stick | R stick | R stick | Mouse |
| LT | L2 (A) | L2 (A) | |
| L1 | L1 | L1 | Ctrl |
| L2 | L2 (D) | L2 (D) | RMB |
| L3 | L3 | L3 | Play/Pause |
| L4 | L4 | L4 | |
| L5 | L5 | L5 | Back |
| RT | R2 (A) | R2 (A) | |
| R1 | R1 | R1 | Alt |
| R2 | R2 (D) | R2 (D) | LMB |
| R3 | R3 | R3 | |
| R4 | R4 | R4 | MMB |
| R5 | R5 | R5 | Forward |
| L pad | Scroll | Scroll | Scroll |
| R pad | Mouse | Mouse | Mouse |
| R pad click | LMB | LMB | LMB |
| Steam | Guide | Home | |
| Steam &rightarrow; L stick up | | | Volume up |
| Steam &rightarrow; L stick down | | | Volume down |
| Steam &rightarrow; L stick left | | | Prev track |
| Steam &rightarrow; L stick right | | | Next track |
| Steam &rightarrow; A | &rightarrow; Nintendo (toggle) | &rightarrow; Xbox (toggle) | &rightarrow; Xbox/Nintendo (toggle) |
| Steam &rightarrow; B | &rightarrow; Lizard (toggle) | &rightarrow; Lizard (toggle) | &rightarrow; Xbox/Nintendo (toggle) |
| Steam &rightarrow; Y | Power off | Power off | Power off |
| Quick Access Menu | &rightarrow; Lizard (hold) | &rightarrow; Lizard (hold) | &rightarrow; Xbox/Nintendo |

### 1. Requirements

#### 1.1 Linux

- `uinput` kernel module

#### 1.2 Windows

- ViGEm by Nefarius, <a href="https://docs.nefarius.at/projects/ViGEm/How-to-Install" title="nefarius ViGEm installation">install guide here</a>

### 2. Installation

#### 2.1 Recommended

Download the latest release and store it in an easy to reach place.
After that, just start it. The tool works, if the trackpads no longer control the mouse and/or a tray icon appears.

## Attribution
<a href="https://www.flaticon.com/free-icons/game-boy-advance" title="game boy advance icons">Game boy advance icons created by Freepik - Flaticon</a>
