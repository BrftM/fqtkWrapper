#!/bin/bash

set -e

# Clean previous files
rm -rf vendor vendor.tar.xz fqtk-bin

# Build fqtk statically and locally
cargo install --locked --root ./fqtk-bin fqtk

# Copy only the binary
mkdir -p vendor
cp fqtk-bin/bin/fqtk vendor/

# Package it
tar -cJf vendor.tar.xz vendor

# Clean up (optional, CI might not need it)
rm -rf vendor fqtk-bin
