extern crate libc;
use libc::c_int;

extern crate itertools;
use itertools::izip;

#[derive(Clone, Default, Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel {
            r : 0,
            g : 0,
            b : 0
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    content: Vec<Pixel>
}

impl Image {
    pub fn from_r_g_b(r : Vec<c_int>, g : Vec<c_int>, b : Vec<c_int>, width : u32, height : u32) -> Image {
        let mut pixel_vec : Vec<Pixel> = Vec::new();
        for (r_elem, g_elem, b_elem) in izip!(&r, &g, &b) {
            pixel_vec.push(Pixel {
                r : *r_elem as u8,
                g : *g_elem as u8,
                b : *b_elem as u8
            });
        }
        Image {
            width,
            height,
            content : pixel_vec
        }
    }

    pub fn to_r_g_b(image : &Image) -> (Vec<c_int>, Vec<c_int>, Vec<c_int>) {
        let mut r : Vec<c_int> = Vec::new();
        let mut g : Vec<c_int> = Vec::new();
        let mut b : Vec<c_int> = Vec::new();
        for pixel in &image.content {
            r.push(pixel.r as c_int);
            g.push(pixel.g as c_int);
            b.push(pixel.b as c_int);
        }
        (r, g, b)
    }
}

