Authors: Alexandre WANG, Pierre DELMER, Enes UYAR, Joris DEMANDRE
# RUST PPM
**A rust project that provides a cli and a python library for reading, writing, and modifying ppm files.**

### REQUIREMENTS
You need to install `cargo` and `rustup` to be able to compile this project. 

More informations here:
 - https://doc.rust-lang.org/cargo/
 - https://www.rust-lang.org/tools/install 
 
You also need python 3 in order to use the library
 
## COMPILING
You can compile the project with this command: `cargo build --release`

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

The library provides tool for reading, writing, grayscaling and inverting ppm files.

You can use it by compiling and then using the target/release/libppm.so

You can also find an example of usage here : [python_demo](https://github.com/demandre/rust-ppm/tree/master/python_demo)
This is the help output of the python demo script - which describe well how it can be used - :
```
usage: ppm.py [-h] -x X -y Y -n NAME

Create an X by Y ppm image file, its grayscaled and inverted version

optional arguments:
  -h, --help            show this help message and exit
  -x X                  Image width
  -y Y                  Image height
  -n NAME, --name NAME  Name of the file to create without file extension
```