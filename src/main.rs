use clap::{App, Arg};

fn main() {

    let matches = App::new("new- cli")
        .version("1.0.0")
        .author("jude omenai")
        .about("parse email")
        .arg(Arg::with_name("Input")
            .help("set file input")
            .index(1))
        .arg(
            Arg::with_name("output")
                .long("output")
                .help("return file in the specified format")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("extended")
                .long("extended")
                .help("check for valid MX")
        )
        .get_matches();

    let output = matches.value_of("output");
    
    println!("the is output, {:?}", output);
    println!("input file {:?}", matches.value_of("Input"));
}


