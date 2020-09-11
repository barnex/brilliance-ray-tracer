# Planetarium

The binary `planetarium` renders the solar system using camera and planet positions from a CSV file.

## Build & Run

[Install Rust](https://www.rust-lang.org/tools/install).

Clone this repo.

Run:
```
cd planetarium
cargo run --release -- my_planets.csv
```

See `cargo run --release -- --help` for command-line options:

```
planetarium 0.1.0
Render a solar system from planet coordinates in CSV format

USAGE:
    planetarium [FLAGS] [OPTIONS] <input>

FLAGS:
        --help       Prints help information
    -t, --topview    Activate top view for debugging
    -V, --version    Prints version information

OPTIONS:
    -e, --every <every>        Only output every nth frame, skip others [default: 1]
        --fov <fov>            Camera field of view (degrees) [default: 60]
    -h, --height <height>      Image height, (pixels) [default: 540]
    -o, --output <output>      Output directory [default: out]
    -s, --samples <samples>    Number of samples per pixel, improves quality [default: 1]
    -w, --width <width>        Image width (pixels) [default: 960]

ARGS:
    <input>    CSV input file
```