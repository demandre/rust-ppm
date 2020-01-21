extern crate libc;
use libc::c_int;

#[link(name = "ppma_io")]
extern {
    fn ppma_example(xsize : c_int, ysize : c_int, r : *mut c_int, g : *mut c_int, b : *mut c_int);
}

pub fn create_example_ppm_wrapper(xsize : i32, ysize : i32) {
    let size = xsize * ysize;
    let mut r = Vec::with_capacity(size as usize);
    let mut g = Vec::with_capacity(size as usize);
    let mut b = Vec::with_capacity(size as usize);
    unsafe {
        ppma_example(
            xsize as c_int,
            ysize as c_int,
            r.as_mut_ptr(),
            g.as_mut_ptr(),
            b.as_mut_ptr()
        );
        r.set_len(size as usize);
        g.set_len(size as usize);
        b.set_len(size as usize);
    }
    println!("{:?}", b);
    println!("{:?}", g);
    println!("{:?}", b);
}
