#!/usr/bin/env bash
set -e

# Prerequisites:
#   cargo install cargo-ndk
#   rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
#   export ANDROID_NDK_HOME=/path/to/android-ndk

OUTPUT="${1:-./jniLibs}"

cargo ndk \
  -t arm64-v8a \
  -t armeabi-v7a \
  -t x86_64 \
  -t x86 \
  -o "$OUTPUT" \
  build --release

echo "Built .so files → $OUTPUT"
