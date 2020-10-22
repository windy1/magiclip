#!/bin/bash

set -e

macos_stop() {
    kill $(pgrep magiclipd)
}

linux_stop() {
    systemctl stop magiclipd
    systemctl status magiclipd
}

if [[ "$OSTYPE" == "darwin"* ]]; then
    macos_stop
elif [[ "$OSTYPE" == "linux-gnu" ]]; then
    linux_stop
else
    echo "Unsupported operating system"
    exit 1
fi
