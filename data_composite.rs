// Datos compuestos
fn main() {
    // Tuplas
    let tup: (i32,i32,f32,&str) = (12,3,62.4,"Hola");
    println!("Los valores de la tupla son {:?}",tup);
    // ¿Y si solo quisieramos imprimir el cuarto valor de la tupla? Pues hacemos los siguiente
    let (x,y,z,t) = tup; // Es obligatorio poner todos las variables de acuerdo al numero de valores que tiene la tupla
    println!("El cuarto valor de la tupla es {}",t); 
    // Segunda manera de obtener un valor de la tupla
    let j = tup.3;
    println!("{}",j);
    // Si quisieramos cambiar algun valor debemos hacer la tupla mutable y aplicar el metodo de arriba
    // tup.2 = "si"; ejemplo


    // Arrays
    let arreglo = [23,24,6,1]; // Los arrays se guardan en el stack y no en el geep
    println!("{:?}",arreglo);
    let arreglo2: [i32; 3] = [34,27,1]; // En los arrays no se pueden guardar datos de diferentes tipos
    println!("{:?}",arreglo2);
    let acc = arreglo2[1]; // Asi podemos acceder a un valor del array
    println!("El valor en la posicion 1 del array es {}",acc);

}