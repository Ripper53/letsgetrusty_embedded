#!/bin/bash

echo "RUNNING run_tests.sh"
set -e  # Exit on error

qemu_command="qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv7m-none-eabi/release/embedded_runner"
list_build_memory="stat -c %s target/thumbv7m-none-eabi/release/embedded_runner | xargs -I{} echo \"SIZE: {}\""

cp embedded_runner/normal_memory.x embedded_runner/memory.x
rm -rf target/thumbv7m-none-eabi
cargo clean
cargo build --target thumbv7m-none-eabi --release
eval "$list_build_memory"

echo "Normal CPU"
eval "$qemu_command"
echo

echo "CPU Throttle"
eval "$qemu_command -icount shift=10,align=on,sleep=on"
echo

echo "Low RAM"
cp -f embedded_runner/low_memory.x embedded_runner/memory.x
rm -rf target/thumbv7m-none-eabi
cargo clean
cargo build --target thumbv7m-none-eabi --release
eval "$qemu_command"
eval "$list_build_memory"

rm -f embedded_runner/memory.x
