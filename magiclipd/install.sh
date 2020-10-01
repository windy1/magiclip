#!/bin/bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

macos_install() {
    DAEMON_PLIST=magiclipd.plist
    TARGET=/Library/LaunchDaemons

    cd $DIR

    cargo clean
    cargo test --release
    cargo install --path .

    cp $DAEMON_PLIST $TARGET
    launchctl unload -w "$TARGET/$DAEMON_PLIST"
    launchctl load -w "$TARGET/$DAEMON_PLIST"
}

if [[ "$OSTYPE" == "darwin"* ]]; then
    macos_install
else
    echo "Unsupported operating system"
    exit 1
fi
