#! /bin/bash

cargo clean
RUSTFLAGS='-C force-frame-pointers=y' cargo build --release
perf record -g --call-graph dwarf ./target/release/brilliance-render
perf report -g,0.3,caller