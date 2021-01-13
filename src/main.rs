
fn main() {

    let arguments = jude_omenai::matches();

    match arguments.value_of("Input"){
        Some(value) => jude_omenai::run(value),
        None =>  println!("name: Jude Omenai, email: jdonmarie@gmail.com")
    }

    
}



