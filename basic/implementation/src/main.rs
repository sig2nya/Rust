struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn say_hello(&self) {
        println!("Hello, I'm {}", self.name);
    }
}

trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Wal Wal!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

fn main() {
    let d = Dog;
    let c = Cat;
    d.speak();
    c.speak();
}
