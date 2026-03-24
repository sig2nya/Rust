use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => panic!("Error occurs : {:?}", error),
    };
}
