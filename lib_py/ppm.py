from cffi import FFI
ffi = FFI()
ffi.cdef("""
    typedef struct {
        int red, int green, int blue;
    } Pixels;

    typedef struct {
        Pixels * pixels,
        int width,
        int height,
        int maxValue,
        char * fileType;
    } Image;

    Image readPPM_libc(char * file_name);

    void writePPM_libc(char * file_name, Image image);


""")

ppm = ffi.dlopen("../target/debug/ppm.dll")