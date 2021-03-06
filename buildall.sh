#!/bin/sh

echo "running library tests..."
cd zmu_cortex_m
echo "TESTING armv6m"
cargo test --features "armv6m generic-device" 
echo "TESTING armv7m"
cargo test --features "armv7m generic-device"
echo "TESTING armv7em"
cargo test --features "armv7em generic-device"
cd ..

cargo build --release --no-default-features --features "armv6m generic-device"
cp ./target/release/zmu ./target/release/zmu-armv6m

cargo build --release --no-default-features --features "armv7m generic-device"
cp ./target/release/zmu ./target/release/zmu-armv7m

cargo build --release --no-default-features --features "armv7em generic-device"
cp ./target/release/zmu ./target/release/zmu-armv7em

cargo build --release --no-default-features --features "armv7em stm32f103" 
cp ./target/release/zmu ./target/release/zmu-stm32f103
