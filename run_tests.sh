#!/bin/bash

echo "RUNNING run_tests.sh"
set -e  # Exit on error

# QEMU simulation command for running release build
qemu_command="qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv7m-none-eabi/release/embedded_runner"
# Command for INFO about memory for the built binary
list_build_memory="stat -c %s target/thumbv7m-none-eabi/release/embedded_runner | xargs -I{} echo \"SIZE: {}\""

# Copy normal QEMU memory config file
cp embedded_runner/normal_memory.x embedded_runner/memory.x
cargo clean
cargo build --target thumbv7m-none-eabi --release
eval "$list_build_memory"

echo "Normal CPU"
eval "$qemu_command"
echo

echo "CPU Throttle"
# Run binary with CPU throttle
eval "$qemu_command -icount shift=10,align=on,sleep=on"
echo

echo "Low RAM"
# Copy low memory QEMU memory config file
cp -f embedded_runner/low_memory.x embedded_runner/memory.x
cargo clean
cargo build --target thumbv7m-none-eabi --release
eval "$qemu_command"
eval "$list_build_memory"

rm -f embedded_runner/memory.x
