# Mac Meeting Auto-Pause

A macOS application that automatically pauses and resumes music playback when your camera turns on/off during meetings.

## Features

- 🎥 Monitors camera status using macOS system logs
- 🎵 Automatically pauses Apple Music and Spotify when camera turns on
- ▶️ Automatically resumes playback when camera turns off
- 🖥️ Clean GUI built with egui showing real-time status
- 📊 Displays camera status, music status, and last event

## How It Works

The application monitors the macOS unified logging system using:
```bash
log stream --predicate 'eventMessage contains "Cameras changed to"'
```

It detects when applications start using the camera (indicating you're in a meeting) by looking for the `appEffects` pattern in the log output.

When an app uses your camera:
- ✅ Camera detected → Music pauses
- ❌ No camera usage → Music resumes

## Build & Run

```bash
# Build the application
cargo build --release

# Run the application
cargo run --release
```

## Requirements

- macOS (tested on macOS Sequoia and later)
- Rust toolchain
- Apple Music or Spotify (or both)

## Supported Music Apps

- Apple Music
- Spotify

The app attempts to control both, so whichever one is running will be paused/resumed.

## Camera Detection

The app detects camera usage by monitoring system logs for the ControlCenter framework's camera events. When an application (like Zoom, Teams, Google Meet, etc.) activates your camera, it triggers a pause. When all apps release the camera, playback resumes.

## GUI

The application shows:
- **Camera Status**: ON (In Meeting) / OFF / Unknown
- **Music Status**: PAUSED / PLAYING
- **Last Event**: Timestamp of the last camera status change

## Notes

- The application needs to keep running in the background to monitor camera status
- Make sure you have music playing in Apple Music or Spotify before testing
- The first time you run it, macOS may ask for permissions to control Music/Spotify via AppleScript
