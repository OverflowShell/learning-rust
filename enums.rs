#[derive(Debug)]
enum So {
    Femenino,
    Masculino
}
#[derive(Debug)]
struct Persona {
    nombre: String,
    genero: So
}
#[derive(Debug)]
enum Algodeloqueseaxd {
    Animal(String), // La manera de definir que sera un string o booleano o entero etc
    Vida(i8)
}

// Uso de type
type Perro = Algodeloqueseaxd;

fn main() {
    let mujer = So::Femenino;
    let hombre = So::Masculino;
    println!("{:?}", mujer);
    println!("{:?}", hombre);
    
    // Vamos a unir la estructura con la enumeracion
    let p1 = Persona {
        nombre: String::from("John"),
        genero: So::Masculino
    };
    println!("{:?}", p1);
    // Usando la palabra instanseada con type
    let nombre = Perro::Animal(String::from("Gatoxd"));
    let vida = Perro::Vida(9);
    println!("{:?}", nombre);
    println!("{:?}", vida);    
}



