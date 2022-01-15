#!/bin/sh
SUDO_BIN=

if [ -z "$SUDO" ]; then 
    SUDO_BIN='/usr/bin/sudo'
else
    SUDO_BIN=$SUDO
fi

cargo build --release

$SUDO_BIN install -Dm755 "target/release/dollah" "/usr/local/bin/"
$SUDO_BIN ln -sf "/usr/local/bin/dollah" "/usr/local/bin/$"

$SUDO_BIN install -Dm755 "target/release/numba" "/usr/local/bin/"
$SUDO_BIN ln -sf "/usr/local/bin/numba" "/usr/local/bin/#"