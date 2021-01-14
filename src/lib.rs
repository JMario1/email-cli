use clap::{App, Arg, ArgMatches};

pub fn matches<'a>() -> ArgMatches<'a>{

    App::new("new-cli")
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

use std::io::Write;
pub fn run(value: &str){

    let content = parse_file(value);
    let is_json = matches().value_of("output")
            .unwrap().contains(".json") ;
    if is_json{
        let out = json_output(&content).unwrap();
        let mut file = fs::File::create(matches().value_of("output").unwrap()).expect("unable to create file");
        
        file.write_all(&out.as_bytes()).expect("could not write to file");

        println!("output file: {:?}", out);
    }

    if matches().is_present("extended") && matches().is_present("output"){
        
        let out = extended(&content);

        let mut file = fs::File::create(matches().value_of("output").unwrap()).expect("unable to create file");

        file.write(b"Email\n").expect("could not write to file");
        file.write_all(&out.iter().as_slice().join("\n").as_bytes()).expect("could not write to file");

        println!("output file: {:?}", out);
    }


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
use resolv::{Resolver, Class, RecordType, Section, Record};
use resolv::record::MX;

#[derive(Serialize)]
struct Object<'a> {
    valid_domains: Vec<& 'a str>,
    total_emails_parsed: u32,
    total_valid_emails: u32,
    categories: HashMap<& 'a str, u32>
}

fn extended(content: &str) -> Vec<&str>{
    
    let re = Regex::new(r"^.+@.+\..+$").unwrap();
    let mut vec: Vec<&str> = Vec::new();
    let mut resolver = Resolver::new().unwrap();

    for line in content.lines().skip(1) {

        if re.is_match(line) && !line.trim().is_empty() {
            let dormain: &str = line.split("@").collect::<Vec<&str>>()[1];
            let mut res = match resolver.query(dormain.as_bytes(), Class::IN, RecordType::MX){
                Ok(v) => v,
                Err(_) => continue
            };
            let mx: Record<MX> = res.get_record(Section::Answer, 0).unwrap();
            if !mx.name.is_empty(){
                vec.push(line.trim())
            }
            
        }
    }

    vec
}

fn json_output(content: &str) -> Result<std::string::String, serde_json::Error>{

    let re = Regex::new(r"^.+@.+\..+$").unwrap();
    let mut vec: Vec<&str> = Vec::new();
    let mut cat: HashMap<&str, u32> = HashMap::new();
    let mut count = 0;
    for line in content.lines().skip(1) {

        if re.is_match(line) {
            let word: Vec<&str> = line.split("@").collect();
            vec.push(word[1])
        }
        if !line.trim().is_empty(){
            count+=1
        };
    }

    for item in vec.iter() {
        let count = cat.entry(item).or_insert(0);
        *count +=1
    }
    let mut vec1 = vec.clone();
    vec1.sort_unstable();
    vec1.dedup();

    let obj = Object{
        valid_domains: vec1,
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
    fn single_email_file(){

       let content =  "Email
       jdon@gmail.com";
        
        
        let out = "\
        {\"valid_domains\":[\"gmail.com\"],\"total_emails_parsed\":1,\"total_valid_emails\":1,\"categories\":{\"gmail.com\":1}}";

        
        assert_eq!(out, json_output(content).unwrap())
    }

    #[test]
    fn email_file_with_newline(){

        let content =  "Emails
        jdon@gmail.com
        
        hell0@gmail.com

        hi@jumia
        ";

        let out = "\
        {\"valid_domains\":[\"gmail.com\"],\"total_emails_parsed\":3,\"total_valid_emails\":2,\"categories\":{\"gmail.com\":2}}";

        
        assert_eq!(out, json_output(content).unwrap())
    }

    #[test]
    fn valid_mx(){
        let inp = "Email
        jd@gmail.com";
        let out = vec!["jd@gmail.com"];
        assert_eq!(extended(inp), out)
    }

    #[test]
    fn invalid_mx(){
        let inp = "Email
        jd@jju.com";
        let out: Vec<&str> = Vec::new();
        assert_eq!(extended(inp), out)
    }
}
