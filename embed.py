from ctypes import cdll

lib = cdll.LoadLibrary("target/debug/libppm.dylib")

image = lib.read_ppm("test.ppm")
print(image)
