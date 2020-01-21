extern crate cc;

fn main() {
    cc::Build::new()
        .file("extern-libs/ppma/ppma_io.c")
        .compile("ppma_io")
}
