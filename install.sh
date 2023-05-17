#!/usr/bin/env bash
rm target/wheels -rf
cargo install --path . 
maturin build --release
pip install target/wheels/plexcryptool-*.whl --force
