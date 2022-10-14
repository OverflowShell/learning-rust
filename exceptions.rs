use std::io::{stdin};


fn main() {
    let mut texto = String::new();
    std::io::stdin()
    .read_line(&mut texto)
    .expect("Error");
    println!("Has ingresado: {}",texto);
}
