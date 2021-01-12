
fn main() {

    let matches = jude_omenai::matches();

    match matches.value_of("Input"){
        Some(value) => jude_omenai::run(value),
        None =>  println!("name: Jude Omenai, email: jdonmarie@gmail.com")
    }

    
}



