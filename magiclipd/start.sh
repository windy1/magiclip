#!/bin/bash

set -e

# macos_start() {
#     # TODO
# }

linux_start() {
    systemctl start magiclipd
    systemctl enable magiclipd
    systemctl status magiclipd
}

if [[ "$OSTYPE" == "darwin"* ]]; then
    macos_start
elif [[ "$OSTYPE" == "linux-gnu" ]]; then
    linux_start
else
    echo "Unsupported operating system"
    exit 1
fi
