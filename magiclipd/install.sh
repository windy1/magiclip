#!/bin/bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cd $DIR

cargo clean
cargo test --release
cargo install --path .

macos_install() {
    DAEMON_PLIST=magiclipd.plist
    TARGET=/Library/LaunchDaemons

    cp $DAEMON_PLIST $TARGET

    launchctl unload -w "$TARGET/$DAEMON_PLIST"
    launchctl load -w "$TARGET/$DAEMON_PLIST"
}

linux_install() {
    SERVICE=magiclipd.service
    TARGET=/etc/systemd/system/

    cp $SERVICE $TARGET

    systemctl daemon-reload
    systemctl enable magiclipd
}

if [[ "$OSTYPE" == "darwin"* ]]; then
    macos_install
elif [[ "$OSTYPE" == "linux-gnu" ]]; then
    linux_install
else
    echo "Unsupported operating system"
    exit 1
fi
