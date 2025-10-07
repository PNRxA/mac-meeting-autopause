# Mac Meeting Auto-Pause

A macOS application that automatically pauses and resumes music playback when your camera turns on/off during meetings.

## Download

Download the latest release from [GitHub Releases](https://github.com/pnrxa/mac-meeting-autopause/releases).

**Note:** This app requires **macOS Sequoia (15.0) or later**.

## Features

- 🎥 Monitors camera status using macOS system logs
- 🎵 Automatically pauses Spotify when camera turns on
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

### For Development
```bash
# Build the application
cargo build --release

# Run the application
cargo run --release
```

### Build macOS App Bundle
To create a standalone `.app` bundle that runs without a terminal window:

```bash
./build-app.sh
```

This creates `Meeting Autopause.app` which you can:
- Double-click to launch
- Move to your Applications folder
- Add to Login Items for automatic startup

## Publishing a New Release

1. Update version in `Cargo.toml`
2. Commit your changes:
   ```bash
   git add .
   git commit -m "Release v1.0.0"
   ```
3. Create and push a new tag:
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin main
   git push origin v1.0.0
   ```
4. Build the app bundle:
   ```bash
   ./build-app.sh
   ```
5. Create a GitHub release with the tag and attach `Meeting Autopause.app` (compressed as `.zip`)

## Requirements

- macOS (tested on macOS Sequoia and later)
- Rust toolchain
- Spotify

## Supported Music Apps

- Spotify

## Camera Detection

The app detects camera usage by monitoring system logs for the ControlCenter framework's camera events. When an application (like Zoom, Teams, Google Meet, etc.) activates your camera, it triggers a pause. When all apps release the camera, playback resumes.

## GUI

The application shows:
- **Camera Status**: ON (In Meeting) / OFF / Unknown
- **Music Status**: PAUSED / PLAYING
- **Last Event**: Timestamp of the last camera status change

## Notes

- The application needs to keep running in the background to monitor camera status
- Make sure you have music playing in Spotify before testing
- The first time you run it, macOS may ask for permissions to control Spotify via AppleScript
