#[derive(Debug)]
struct Mult {
    ancho: u32,
    largo: u32
}

impl Mult {
    fn area(&self) -> u32 {
        self.ancho * self.largo
    }
}

fn main() {
    let si = Mult {
        ancho: 50,
        largo: 30
    };
    println!("El area es {:?}", si.area());
}