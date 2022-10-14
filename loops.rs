fn main() {
    let mut x: u32 = 34;
    // Ciclo loop
    if x == 34 {
        loop {
            println!("Hola");
            break
        };
    }
    else {
        println!("El numero no es 34");
    }

    x = loop {
        break 2 * 5; // Break tambien sirve como un return pero en bucles
    };
    println!("{}",x);

    // Ciclo While

    while x == 10 {
        println!("10");
        break
    }

    // Ciclo for

    let array: [i32; 4] = [125,462,12,743];
    for iterar in array.iter() {
        println!("El valor es {}",iterar);
    }    

    // Iterar con while

    let y = 5;
    while y < 10 {
        println!("El valor es {}",array[3]);
        break 
    }

    // Iterar hasta un valor

    for iterar in 1..4 {
        println!("El valor es {}",iterar);
    } // Esto es como hacer un for i in range(1,4) en python

    // En reversa

    for iterar in (1..4).rev() {
        println!("El valor es {}",iterar);
    } 
}