mod ppma_wrapper;
use ppma_wrapper::image::{Invertable, Grayscalable, Image};

#[no_mangle]
pub extern fn dummy () -> u8 {
    return 42;
}

pub extern fn read_ppm(input_name : String) -> Image {
    ppma_wrapper::ppma_read_wrapper(input_name)
}

pub extern fn write_ppm(file_out_name : String, image: Image) {
    ppma_wrapper::ppma_write_wrapper(file_out_name, image)
}
