#!/bin/bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
USER=${SUDO_USER:-$USER}

cd $DIR

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

if [[ "$OSTYPE" == "darwin"* ]] || [[ "$OSTYPE" == "linux-gnu" ]]; then
    cargo install --path .
else
    echo "Unsupported operating system"
    exit 1
fi

if [[ "$OSTYPE" == "linux-gnu" ]]; then
    linux_install
fi
