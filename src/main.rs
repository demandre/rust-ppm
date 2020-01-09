extern crate clap; 
use clap::{Arg, App};

#[derive(Copy, Clone, Default)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

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
            .required(true))
       .get_matches(); 

    let input = matches.value_of("in").unwrap();

    println!("{}",input);
}