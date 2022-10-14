// Funcion con parametros
fn main() {
    mi_funcion(10,20);
    let x = get();
    println!("{}",x);
    variables();
}   
fn mi_funcion(x: i32,y: i32) {
    println!("El valor de la variable x es {}",x);
    println!("El valor de la variable y es {}",y);
}
// Funcion de retorno de un valor 
fn get() -> i32 { // Con -> i32 le indicamos que esta funcion va a retornar un entero de 32 bits
    return 10; // Return funciona igual que python
}
// Funcion de problema matematico
fn variables() {
    let z = 5;
    let y = {
        let z = 8; // Declaramos otra variable llamada z y veremos que no se altera el valor del z original porque esta dentro de y variable
        z * 5 // Aqui no se debe poner el ;
    };
    println!("El valor de z es {}",z);
    println!("El valor de z es {}",y);
}