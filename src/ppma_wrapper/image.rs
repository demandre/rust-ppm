extern crate libc;
use libc::c_int;

extern crate itertools;
use itertools::izip;

/// RGB pixel struct : 
/// * r : byte for red
/// * g : byte for green
/// * b : byte for blue
#[derive(Clone, Default, Debug, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    /// Returns a Pixel with r, g and b bytes given
    ///
    ///  # Arguments
    ///
    ///  * `r` : red byte
    ///  * `g` : green byte
    ///  * `b` : blue byte
    ///
    ///  # Example
    ///
    ///  ```
    ///  use image::Pixel
    ///  let pixel = Pixel::new(10, 50, 200);
    ///  ```
    pub fn new(r : u8, g : u8, b : u8) -> Pixel {
        Pixel {
            r,
            g,
            b
        }
    }
}

pub trait Invertable {
    fn invert(&mut self);
}

pub trait Grayscalable {
    fn grayscale(&mut self);
    fn grayscale_luma(&mut self);
}

/// Image struct :
/// * width: image width
/// * height: image height
/// * content: image pixels
#[derive(Clone, Default, Debug)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    content: Vec<Pixel>
}

impl Image {
    /// Convert r,g,b vectors into an Image struct
    /// 
    /// # Arguments:
    /// * `r` : red byte vector
    /// * `g` : green byte vector
    /// * `b` : blue byte vector
    /// * `width` : number of pixels per line
    /// * `height` : number of lines
    ///
    /// # Example
    ///
    /// ```
    /// use image::Image;
    /// let r = vec![0, 1, 2, 3];
    /// let g = vec![0, 1, 2, 3];
    /// let b = vec![0, 1, 2, 3];
    /// let image = Image::from_r_g_b(r, g, b, 2, 2);
    /// ```
    pub fn from_r_g_b(r : Vec<c_int>, g : Vec<c_int>, b : Vec<c_int>, width : u32, height : u32) -> Image {
        let mut pixel_vec : Vec<Pixel> = Vec::new();
        for (r_elem, g_elem, b_elem) in izip!(&r, &g, &b) {
            pixel_vec.push(Pixel::new(
                *r_elem as u8,
                *g_elem as u8,
                *b_elem as u8
            ));
        }
        Image {
            width,
            height,
            content : pixel_vec
        }
    }

    /// Convert Image struct to r, g, b vectors
    ///
    /// # Arguments:
    /// * `image` : borrowed pointer to the image struct
    ///
    /// # Example
    ///
    /// ```
    /// use image::{Image, Pixel};
    /// let image = {
    ///     width: 1,
    ///     height: 1,
    ///     content: vec![Pixel::new(0, 0, 0)]
    /// };
    /// let (r, g, b) = Image::to_r_g_b(&image);
    /// ```
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

impl Invertable for Image {
    /// Invert colors of an image
    ///
    /// # Example
    ///
    /// ```
    /// use image::{Image, Pixel};
    /// let mut image = {
    ///     width: 1,
    ///     height: 1,
    ///     content: vec![Pixel::new(0, 0, 0)]
    /// };
    /// image.invert();
    /// ```
    fn invert(&mut self) {
        for mut pixel in &mut self.content {
            pixel.r = 255-pixel.r;
            pixel.g = 255-pixel.g;
            pixel.b = 255-pixel.b;
        }
    }
}

impl Grayscalable for Image {
    /// Grayscale an image
    ///
    /// # Example
    ///
    /// ```
    /// use image::{Image, Pixel};
    /// let mut image = {
    ///     width: 1,
    ///     height: 1,
    ///     content: vec![Pixel::new(0, 0, 0)]
    /// }
    /// image.grayscale();
    /// ```
    fn grayscale(&mut self) {
        for mut pixel in &mut self.content {
            let average = pixel.r/3 + pixel.g/3 + pixel.b/3;
            pixel.r = average;
            pixel.g = average;
            pixel.b = average;
        }
    }

    fn grayscale_luma(&mut self) {
        for mut pixel in &mut self.content {
            let average = pixel.r as f32 *0.3 + pixel.g as f32 *0.59 + pixel.b as f32 *0.11;
            pixel.r = average as u8;
            pixel.g = average as u8;
            pixel.b = average as u8;
        }
    }
}

