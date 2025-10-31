use std::collections::HashMap;

enum IpAddr {
    V4(String),
    V6(String),
}

struct User {
    id: u32,
    name: String,
    active: bool,
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    }
    else {
        Ok(a / b)
    }
}

fn main() {
    let a: i32 = -10;
    let b: u64 = 123;
    let size = std::mem::size_of::<i64>();
    let x = 2.5;
    let y: f32 = 3.14;
    let t = true;
    let f: bool = false;
    let c = 'A';
    let tup: (i32, f64, &str) = (500, 6.4, "hello");
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("tup.0 = {}", tup.0);
    let arr: [u8, 4] = [0, 1, 2, 3];
    let s1 = "Hello";
    let mut s2 = String::from("Rust");
    s2.push('!');
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let user = User {
        id: 1,
        name: String::from("sig"),
        active: true,
    };

    let some_val: Option<i32> = Some(10);
    let none_val: Option<i32> = None;
}
