#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

RUST_BACKTRACE=1 cargo build -p foundry-swift

swiftc -L ../../target/debug \
  -lfoundry_swift \
  -import-objc-header bridging-header.h \
  main.swift ./generated/SwiftBridgeCore.swift ./generated/foundry-swift/foundry-swift.swift -framework Foundation -framework SystemConfiguration
