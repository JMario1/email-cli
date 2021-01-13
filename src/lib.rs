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

    let content = parse_file(value);
    let is_json = matches().value_of("output")
            .unwrap().contains(".json") ;
    if is_json{
        check_valid_email(&content).unwrap();
    }

    println!("input file {:?}", value);
}

use std::fs;
use std::process;

fn parse_file(file: &str) -> String {
    let content = fs::read_to_string(file).unwrap_or_else(|err| {
        println!("something went wrong while opening file {}", err);
        process::exit(1)

    });
    content
    
}

use std::collections::HashMap;
use serde::Serialize;
use regex::Regex;
use serde_json;

#[derive(Serialize)]
struct Object<'a> {
    valid_domains: Vec<& 'a str>,
    total_emails_parsed: u32,
    total_valid_emails: u32,
    categories: HashMap<& 'a str, u32>
}


fn check_valid_email(content: &str) -> Result<std::string::String, serde_json::Error>{

    let re = Regex::new(r"^.+@.+\..+$").unwrap();
    let mut vec: Vec<&str> = Vec::new();
    let mut cat: HashMap<&str, u32> = HashMap::new();
    let mut count = 0;
    for line in content.lines() {

        if re.is_match(line) {
            let word: Vec<&str> = line.split("@").collect();
            vec.push(word[1])
        }
        count +=1;
    }

    for item in vec.iter() {
        let count = cat.entry(item).or_insert(0);
        *count +=1
    }

    let obj = Object{
        valid_domains: vec.clone(),
        total_emails_parsed: count,
        total_valid_emails: vec.len() as u32,
        categories: cat
    };
    let out = serde_json::to_string(&obj);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    
    
    #[test]
    fn json_output(){

       let content =  "jdon@gmail.com";
        
        
        let out = "\
        {\"valid_domains\":[\"gmail.com\"],\"total_emails_parsed\":1,\"total_valid_emails\":1,\"categories\":{\"gmail.com\":1}}";

        println!("out {:?}", check_valid_email(content).unwrap());
        assert_eq!(out, check_valid_email(content).unwrap())
    }
}
