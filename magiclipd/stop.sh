#!/bin/bash

set -e

macos_stop() {
    kill $(pgrep magiclipd)
}

if [[ "$OSTYPE" == "darwin"* ]]; then
    macos_stop
elif [[ "$OSTYPE" == "linux-gnu" ]]; then
    linux_stop
else
    echo "Unsupported operating system"
    exit 1
fi
