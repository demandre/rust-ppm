extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod ppma_wrapper;
use ppma_wrapper::image::{Invertable, Grayscalable, Image};

#[no_mangle]
#[pyfunction]
pub extern fn dummy () -> u8 {
    return 42;
}

#[no_mangle]
#[pyfunction]
pub extern fn read_ppm(input_name : String) ->  PyResult<Image> {
    Ok(ppma_wrapper::ppma_read_wrapper(input_name))
}

#[no_mangle]
#[pyfunction]
pub extern fn write_ppm(file_out_name : String, image: Image) {
    ppma_wrapper::ppma_write_wrapper(file_out_name, image)
}

#[no_mangle]
#[pyfunction]
pub extern fn create_ppm(xsize : i32, ysize : i32) -> PyResult<Image> {
    Ok(ppma_wrapper::create_example_ppm_wrapper(xsize, ysize))
}

#[no_mangle]
#[pyfunction]
pub extern fn invert(image : Image) -> PyResult<Image> {
    Ok(ppma_wrapper::invert(image))
}

#[no_mangle]
#[pyfunction]
pub extern fn grayscale(image : Image) -> PyResult<Image> {
    Ok(ppma_wrapper::grayscale(image))
}

#[pymodule]
fn ppm(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_ppm))?;
    m.add_wrapped(wrap_pyfunction!(write_ppm))?;
    m.add_wrapped(wrap_pyfunction!(create_ppm))?;
    m.add_wrapped(wrap_pyfunction!(dummy))?;
    m.add_wrapped(wrap_pyfunction!(invert))?;
    m.add_wrapped(wrap_pyfunction!(grayscale))?;

    Ok(())
}
