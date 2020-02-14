Authors: Alexandre WANG, Pierre DELMER, Enes UYAR, Joris DEMANDRE
# RUST PPM
**A rust project that provides a cli and a python library for reading, writing, and modifying ppm files.**

### REQUIREMENTS
You need to install `cargo` and `rustup` to be able to compile this project. 

More informations here:
 - https://doc.rust-lang.org/cargo/
 - https://www.rust-lang.org/tools/install 
 
## COMPILING
You can compile the project with this command: `cargo build`

## CLI

The cli provides a tool which take a file ppm as input and then inverts and grayscales it with output prefix given.

This is the help output of the cli - which describe well how it can be used - :
```
ppm 1.0
Wang A., Delmer P., Uyar E., Demandre J.
PPM utility

USAGE:
    ppm [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --in <INPUT_PPM_FILE>                    Sets input file
    -o, --out-prefix <OUTPUT_PPM_FILE_PREFIX>    Sets output file prefix
```

## PYTHON LIBRARY

The library provides tool for reading and writing ppm files.

You can use it like it's used in [embed.py](https://github.com/demandre/rust-ppm/blob/feature/ImageModifications/embed.py)
