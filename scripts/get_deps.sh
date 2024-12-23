#!/bin/bash

AX_ROOT=.arceos

test ! -d "$AX_ROOT" && echo "Cloning repositories ..." || true
test ! -d "$AX_ROOT" && git clone https://github.com/arceos-org/arceos "$AX_ROOT" --depth=1 || true

echo "Copying Cargo.lock ..."
cp "$AX_ROOT/Cargo.lock" Cargo.lock

$(dirname $0)/set_ax_root.sh $AX_ROOT
