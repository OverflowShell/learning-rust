fn main() {
    let mut saludo = String::from("String with String function");
    println!("{}",saludo.capacity());
    let mut saludo1: &str = "Hola";
    // println!("{}",saludo1.capacity());
    // Con &str no podemos ver cuanta capacidad tiene porque el compilador no sabe cuanta memoria tiene
    // Cosa que no pasa cuando usamos String porque reserva la memoria dinamicamente la lee y le da un espacio,reasigna y quita etc

    // Ejemplo de reasignamiento dinamico que usa String

    saludo.push_str(" desde rust"); // Con &str no podemos usar esta funcion para concatenar
    println!("{}",saludo);
    println!("{}",saludo.capacity());

    si(saludo);

}

// Como pasar con String de funcion a funcion sin usar &str
fn si(saludo: String) { // Si pusieramos String de este modo usando &str nos daria error
    println!("{}",saludo); // &str con &str siempre y String con String
}