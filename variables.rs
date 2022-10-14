/*
Tipos de datos en rust
i8,i32,i64 = integer es decir numero positivo o negativo de tantos bits
f8,f32,f64 = flotante de tantos bits
u8,u32,u64 = unsigned es decir numeros positivos 

*/

fn main() {

    
    println!("Tipos de variables");
    let si: i32 = 3444; // Variable inmutable - no mutable
    println!("Variable no mutable con let {}",si);
    let mut a: i64 = 34; // Variable mutable (mut)
    println!("Variable inicial {}",a);
    a = 33; // Modificamos la variable
    println!("Variable modificada {}",a);
    println!("Accedemos a la direccion de memoria de la variable modificada a traves de otra variable");
    let b = &a;
    println!("La variable es {} y su direccion de memoria es {:p}",b,b);
    println!("Constantes");
    // Las constantes deben ser en mayusculas
    const CONSTANTE: u32 = 100_00; // 100,00 o 100.00 se le debe poner "_"
    println!("El valor de la constante es {}",CONSTANTE);
    // Calcularemos el tamaño de una variable
    // A esto se le llama sombreado 
    let x = "Hola mundo";
    println!("{}",x);
    let x = x.len(); // len() sirve para decirnos la longitud de un string
    // Si le pondriamos mut le indicariamos que va a cambiar el valor y nos retornaria un error por ser un string y querer cambiarlo a un entero
    println!("La frase tiene {} de letras",x);

}
