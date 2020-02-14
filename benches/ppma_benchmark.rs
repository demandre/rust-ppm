use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../src/ppma_wrapper.rs"]
mod ppma_wrapper;
use ppma_wrapper::image::Invertable;
use ppma_wrapper::image::Grayscalable;

fn test_libraries() {
    let image = ppma_wrapper::create_example_ppm_wrapper(100, 100);
    ppma_wrapper::ppma_write_wrapper(String::from("test.ppm"), image.clone());

    let mut image_test_to_invert = image.clone();
    image_test_to_invert.invert();
    ppma_wrapper::ppma_write_wrapper(String::from("testInverted.ppm"), image_test_to_invert);

    let mut image_test_to_grayscale = image.clone();
    image_test_to_grayscale.grayscale_luma();
    ppma_wrapper::ppma_write_wrapper(String::from("testGreyscaled.ppm"), image_test_to_grayscale);
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test_libraries", |b| b.iter(|| test_libraries()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
