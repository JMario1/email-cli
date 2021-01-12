use clap::{App, Arg, ArgMatches};

pub fn matches<'a>() -> ArgMatches<'a>{

    App::new("new- cli")
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
    .get_matches()
}

pub fn run(value: &str){
    println!("input file {:?}", value);
}
