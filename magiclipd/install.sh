#!/bin/bash

set -ex

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
USER=${SUDO_USER:-$USER}

cd $DIR

cargo install --path .

macos_install() {
    DAEMON_TARGET=/Library/LaunchDaemons/magiclipd.plist

    launchctl unload -w $DAEMON_TARGET || echo "No previous installation to unload"

    cat << EOF > $DAEMON_TARGET
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>magiclipd</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>RUST_LOG</key>
        <string>debug</string>
    </dict>
    <key>ProgramArguments</key>
    <array>
        <string>$HOME/.cargo/bin/magiclipd</string>
    </array>
    <key>UserName</key>
    <string>$USER</string>
    <key>StandardOutPath</key>
    <string>$HOME/.magiclip/magiclipd.log</string>
    <key>StandardErrorPath</key>
    <string>$HOME/.magiclip/magiclipd.log</string>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
EOF

    launchctl load -w $DAEMON_TARGET
}

linux_install() {
    SERVICE_TARGET=/lib/systemd/system/magiclipd.service

    systemctl stop magiclipd || echo "No previous installation to stop"

    cat << EOF > $SERVICE_TARGET
[Unit]
Description=Magiclip service daemon
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=always
RestartSec=1
User=$USER
Environment=RUST_LOG=debug
Environment=DISPLAY=$DISPLAY
ExecStart=$HOME/.cargo/bin/magiclipd

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    systemctl start magiclipd
    systemctl enable magiclipd
    systemctl status magiclipd
}

if [[ "$OSTYPE" == "darwin"* ]]; then
    macos_install
elif [[ "$OSTYPE" == "linux-gnu" ]]; then
    linux_install
else
    echo "Unsupported operating system"
    exit 1
fi
