extern crate clap;

mod ppma_wrapper;
use ppma_wrapper::image::Invertable;
use ppma_wrapper::image::Grayscalable;
use clap::{Arg, App};

fn main() { 
    let matches = App::new("ppm")
       .version("1.0")
       .about("PPM utility")
       .author("Demandre J.")
       .arg(Arg::with_name("in")
            .short("in")
            .long("in")
            .value_name("INPUT_PPM_FILE")
            .help("Sets input file")
            .takes_value(true)
            .required(false))
       .arg(Arg::with_name("out-prefix")
            .short("out-prefix")
            .long("out-prefix")
            .value_name("OUTPUT_PPM_FILE_PREFIX")
            .help("Sets output file prefix")
            .takes_value(true)
            .required(false))
       .get_matches(); 

    let input = matches.value_of("in");
    let output_prefix = matches.value_of("out-prefix");

    if input != None {
        println!("opening {} image",input.unwrap());
        
        let mut image = ppma_wrapper::ppma_read_wrapper(String::from(input.unwrap()));
        ppma_wrapper::ppma_write_wrapper(String::from(output_prefix.unwrap().to_owned() + ".ppm"), image.clone());

        let mut inverted_image = image.clone();
        inverted_image.invert();    
        ppma_wrapper::ppma_write_wrapper(String::from(output_prefix.unwrap().to_owned() + "Inverted.ppm"), inverted_image);

        let mut grayscaled_image = image.clone();
        grayscaled_image.grayscale_luma();    
        ppma_wrapper::ppma_write_wrapper(String::from(output_prefix.unwrap().to_owned() + "Grayscaled.ppm"), grayscaled_image);
    } else {
        testLibraries();
    }
}

fn testLibraries() {
    let mut image = ppma_wrapper::create_example_ppm_wrapper(1000, 1000);
    ppma_wrapper::ppma_write_wrapper(String::from("test.ppm"), image.clone());

    let mut image_test_to_invert = image.clone();
    image_test_to_invert.invert();
    ppma_wrapper::ppma_write_wrapper(String::from("testInverted.ppm"), image_test_to_invert);

    let mut image_test_to_grayscale = image.clone();
    image_test_to_grayscale.grayscale_luma();
    ppma_wrapper::ppma_write_wrapper(String::from("testGreyscaled.ppm"), image_test_to_grayscale);
}
