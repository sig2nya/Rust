trait Speak {
    fn say(&self);
}

struct Dog;

impl Speak for Dog {
    fn say(&self) { println!("Wal Wal!"); }
}

struct Cat;

impl Speak for Cat {
    fn say(&self) { println!("Meow~"); }
}

fn main() {
    let animals: Vec<Box<dyn Speak>> = vec![
        Box::new(Dog), // for Heap Mem
        Box::new(Cat),
    ];

    for a in animals {
        a.say(); // dynamic dispatch
    }

    let x = 10;
    let r: *const i32 = &x; // read & write possible

    unsafe {
        println!("r points to {}", *r);
    }
}
