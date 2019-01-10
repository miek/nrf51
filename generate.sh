#!/usr/bin/env bash
rm -r src/
python scripts/svdpatch.py nrf51.yaml
svd2rust -i nrf51.svd.patched
rustfmt lib.rs
form -i lib.rs -o src/
cargo fmt
rm lib.rs
