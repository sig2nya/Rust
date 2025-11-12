#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct User {
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn is_active(&self) -> bool {
        self.active
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
        println!("Meow Meow!");
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {} square pixels", rect1.area());

    let mut user = User::new("Hong".into(), "hong@example.com".into());
    println!("active? {}", user.is_active()); // true

    user.deactivate();
    println!("active? {}", user.is_active()); // false

    let d = Dog;
    let c = Cat;

    d.speak();
    c.speak();
}
