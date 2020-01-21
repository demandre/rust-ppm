#! /bin/bash
#
cp ppma_io.h /$HOME/Documents/programmes/rust/rust-ppm/extern-libs/ppma/bin
#
gcc -c -Wall -I /$HOME/Documents/programmes/rust/rust-ppm/extern-libs/ppma/bin ppma_io.c
if [ $? -ne 0 ]; then
  echo "Compile error."
  exit
fi
#
mv ppma_io.o ~/Documents/programmes/rust/rust-ppm/extern-libs/ppma/bin/ppma_io.o
#
echo "Normal end of execution."
