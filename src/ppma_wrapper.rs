extern crate libc;
use libc::{c_int, c_char};

pub mod image;
pub use image::Image;

use std::ffi::CString;
use std::ptr;


#[link(name = "ppma_io")]
extern {
    fn ppma_example(xsize : c_int, ysize : c_int, r : *mut c_int, g : *mut c_int, b : *mut c_int);
    fn ppma_write(file_out_name : *const c_char, xsize : c_int, ysize : c_int, r : *const c_int, g : *const c_int, b : *const c_int);
    fn ppma_read(input_name : *const c_char, xsize : *mut c_int, ysize : *mut c_int, rgb_max : *mut c_int, r : *mut *mut c_int, g : *mut *mut c_int, b : *mut *mut c_int);
}

macro_rules! retrieve {
    ($size : expr; $($x : expr),*) => {
        $(
            $x.set_len($size);
        )*
    }
}

macro_rules! push {
    ($($elem : expr),* ; $($vector : expr),*) => {
        $(
            $vector.push($elem);
        )*
    }
}

macro_rules! offset {
    ($($p : expr),*) => {
        $(
            $p = $p.offset(1);
        )*
    }
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
        retrieve!(size; r, g, b);
    }
    Image::from_r_g_b(r, g, b, xsize as u32, ysize as u32)
}

pub fn ppma_write_wrapper(file_out_name : String, image : Image) {
    let (r, g, b) = Image::to_r_g_b(&image);
    unsafe {
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

pub fn ppma_read_wrapper(input_name : String) -> Image {
    let (mut x, mut y, mut rgb_max) = init_r_g_b(1);
    let mut r = Vec::with_capacity(1);
    let mut g = Vec::with_capacity(1);
    let mut b = Vec::with_capacity(1);
    let mut vec_r : Vec<c_int> = vec![];
    let mut vec_g : Vec<c_int> = vec![];
    let mut vec_b : Vec<c_int> = vec![];
    unsafe {
        ppma_read(
            CString::new(input_name).expect("CString::new failed").as_ptr(),
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            rgb_max.as_mut_ptr(),
            r.as_mut_ptr(),
            g.as_mut_ptr(),
            b.as_mut_ptr()
        );
        retrieve!(1; x, y, rgb_max, r, g, b);
        let size = x[0] * y[0];
        let mut p_r = r[0];
        let mut p_g = g[0];
        let mut p_b = b[0];
        for _i in 0..size {
            push!(*p_r, *p_g, *p_b; vec_r, vec_g, vec_b);
            offset!(p_r, p_b, p_g);
        }
    }
    Image::from_r_g_b(vec_r, vec_g, vec_b, x[0] as u32, y[0] as u32)
}
