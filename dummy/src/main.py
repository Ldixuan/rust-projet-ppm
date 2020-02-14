from cffi import FFI
ffi = FFI()
ffi.cdef("""
    int dummy();
""")

lib = ffi.dlopen("../target/debug/dummy.dll")

print(lib.dummy())
