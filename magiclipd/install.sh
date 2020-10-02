#!/bin/bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cd $DIR

cargo clean
cargo test --release

macos_install() {

    DAEMON_PLIST=magiclipd.plist
    TARGET=/Library/LaunchDaemons

    cargo install --path .
    cp $DAEMON_PLIST $TARGET

    launchctl unload -w "$TARGET/$DAEMON_PLIST"
    launchctl load -w "$TARGET/$DAEMON_PLIST"
}

linux_install() {
    SERVICE=magiclipd.service
    TARGET=/lib/systemd/system/

    cargo build --release
    cp ../target/release/magiclipd /usr/bin/
    systemctl stop magiclipd || echo "No previous installation to stop"

    cp $SERVICE $TARGET

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
