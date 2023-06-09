# plexcryptool

A collection of tools for cryptography uses.

# Hybrid Repository
This Repository uses a combination of python and rust code through pyo3 rust bindings.
The reason for this is that python is a tool dynamic language, 
screwing around with datatypes a bit too much for my liking.

# Compiling
Parts of the python scripts in this repository use my library plexcryptool,
which is implemented in Rust. To compile follow [this guide](https://pyo3.rs/main/getting_started)
It boils down to the following steps:
- [Install Rust](https://www.rust-lang.org/tools/install), preferably through rustup, as that is the official Rust distribution. Your package manager might still be fine.
- Make sure you use the right Python version. I made this with Python 3.11 and PyO3 requires at least Python 3.7
- Create a virtual environment in the root of the repository. I used `python -m venv .venv` for this. Activate the venv.
- Install maturin `pip install maturin --user`
- compile the plexcryptool python module using `maturin develop -r` or `maturin build --release`
- install the module with pip: `pip install target/wheels/plexcryptool-X.X.X-cp311-cp311-manylinux_2_34_x86_64.whl`
- compile the plexcryptool executable using `cargo run --release` or install it to your system with `cargo install --path  .`

Alternatively, you can use the install.sh script to install both the executable and python (pip) package to your system.

Thats it!

# License
MIT License

# Mirrored
This Repository is mirrored from my personal [Git Server](https://git.cscherr.de).

The upstream is [git.cscherr.de/PlexSheep/plexcryptool/](https://git.cscherr.de/PlexSheep/plexcryptool/).
