#[derive(Debug)]
struct Pe {
    name: String,
    nepe: String
}

impl Pe {
    fn xd(&self) {
        println!("{} {}", self.name, self.nepe);
    }
}

fn main() {
    let a = Pe {
        name: String::from("Desmon"),
        nepe: String::from("chupa pene")
    };
    a.xd();
}
