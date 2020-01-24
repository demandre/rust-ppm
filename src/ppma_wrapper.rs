extern crate libc;
use libc::{c_int, c_char};

pub mod image;
pub use image::Image;

use std::ffi::CString;


#[link(name = "ppma_io")]
extern {
    fn ppma_example(xsize : c_int, ysize : c_int, r : *mut c_int, g : *mut c_int, b : *mut c_int);
    fn ppma_write(file_out_name : *const c_char, xsize : c_int, ysize : c_int, r : *const c_int, g : *const c_int, b : *const c_int);
}

fn init_r_g_b(size : usize) -> (Vec<c_int>, Vec<c_int>, Vec<c_int>){
    let r = Vec::with_capacity(size);
    let g = Vec::with_capacity(size);
    let b = Vec::with_capacity(size);
    (r, g, b)
}

pub fn create_example_ppm_wrapper(xsize : i32, ysize : i32) -> Image {
    let size = (xsize * ysize) as usize;
    let (mut r, mut g, mut b) = init_r_g_b(size);
    unsafe {
        ppma_example(
            xsize as c_int,
            ysize as c_int,
            r.as_mut_ptr(),
            g.as_mut_ptr(),
            b.as_mut_ptr()
        );
        r.set_len(size);
        g.set_len(size);
        b.set_len(size);
    }
    Image::from_r_g_b(r, g, b, xsize as u32, ysize as u32)
}

pub fn ppma_write_wrapper(file_out_name : String, image : Image) {
    let (r, g, b) = Image::to_r_g_b(&image);
    unsafe{
        ppma_write(
            CString::new(file_out_name).expect("CString::new failed").as_ptr(),
            image.width as c_int,
            image.height as c_int,
            r.as_ptr(),
            g.as_ptr(),
            b.as_ptr()
        );
    }
}
