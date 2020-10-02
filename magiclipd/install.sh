#!/bin/bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
BIN=../target/release/magiclipd
# USER=${SUDO_USER:-$USER}

cd $DIR

cargo build --release

macos_install() {
    DAEMON=magiclipd.plist
    DAEMON_TARGET=/Library/LaunchDaemons
    BIN_TARGET=/usr/local/bin/

    cp $BIN $BIN_TARGET
    cp $DAEMON $DAEMON_TARGET

    launchctl unload -w "$TARGET/$DAEMON_PLIST" || echo "No previous installation to unload"
    launchctl load -w "$TARGET/$DAEMON_PLIST"
}

linux_install() {
    SERVICE=magiclipd.service
    SERVICE_TARGET=/lib/systemd/system/
    BIN_TARGET=/usr/bin/

    systemctl stop magiclipd || echo "No previous installation to stop"
    cp $BIN $BIN_TARGET
    cp $SERVICE $SERVICE_TARGET

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
