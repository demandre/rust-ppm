from ctypes import cdll

lib = cdll.LoadLibrary("target/debug/libppm.rlib")

print(lib.dummy())
