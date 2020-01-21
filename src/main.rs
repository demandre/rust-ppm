extern crate clap;

mod ppma_wrapper;

use clap::{Arg, App};


fn main() { 
    /*let matches = App::new("ppm")
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
       .get_matches(); 
    let input = matches.value_of("in");

    if input != None {
        println!("{}",input.unwrap());
    } else {
        // Test Pixel struct
        let pixel = Pixel { r: 42, ..Default::default() };

        println!("{:?}",pixel);

        // Test Image struct
        let pixel2 = Pixel { r: 43, ..Default::default() };
        let mut image = Image { width:242, content: vec![], ..Default::default() };
        image.content.push(pixel);
        image.content.push(pixel2);

        println!("{:?}",image);
    }*/
    let image_test = ppma_wrapper::create_example_ppm_wrapper(10, 10);
    ppma_wrapper::ppma_write_wrapper(String::from("test.ppm"), image_test);
}
