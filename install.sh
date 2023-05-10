#!/usr/bin/env bash
rm target/wheels -rf
cargo install --path . 
maturin build --release
pip install target/wheels/plexcryptool*x86_64.whl --force
