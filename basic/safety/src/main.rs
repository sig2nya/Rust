fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot Devide by 0"))
    }
    else {
        Ok(a / b)
    }
}

fn read_file() -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(Contents)
}

fn main() {
    let result = divide(10, 2);

    match result {
        Ok(value) => println!("Result : {}", value),
        Err(err_msg) => println!("Error : {}", err_msg),
    }
}
