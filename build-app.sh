#!/bin/bash

# Build the release binary
echo "Building release binary..."
cargo build --release

# Create app bundle structure
APP_NAME="Meeting Autopause.app"
echo "Creating app bundle structure..."
rm -rf "$APP_NAME"
mkdir -p "$APP_NAME/Contents/MacOS"
mkdir -p "$APP_NAME/Contents/Resources"

# Copy the binary
echo "Copying binary..."
cp target/release/mac-meeting-autopause "$APP_NAME/Contents/MacOS/"

# Create Info.plist
echo "Creating Info.plist..."
cat > "$APP_NAME/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>mac-meeting-autopause</string>
    <key>CFBundleName</key>
    <string>Meeting Autopause</string>
    <key>CFBundleIdentifier</key>
    <string>com.meetingautopause.app</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSUIElement</key>
    <false/>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

echo "✅ App bundle created: $APP_NAME"
