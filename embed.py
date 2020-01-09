from ctypes import cdll

lib = cdll.LoadLibrary("target/debug/libppm.dylib")

print(lib.dummy())