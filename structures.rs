// Estructuras en rust1
#[derive(Debug)]
struct Name {
    usuario: String,
    correo: String,
    veces_activo: u64,
    activo: bool
}
#[derive(Debug)]
struct Color(i32, i32, i32); // Definimos desde antes el tipo de datos en la estructura

fn main() {
    let mut variable = Name {
        correo: String::from("si@gmail.com"),
        usuario: String::from("Dennise"),
        veces_activo: 1,
        activo: true
    };
    println!("{:?}", variable); // Imprimimos todo
    println!("{}", variable.correo); // Imprimimos solamente el correo
    variable.usuario = String::from("Codigo"); // Cambiamos el valor de usuario
    println!("{}", variable.usuario);

    // Struct color
    let color = Color(3, 83, 245);
    println!("{:?}", color);
}