fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 : {}", s1); Error!! value borrowed here after move

    calculate_length(&s1);

    println!("s2 : {}", s2);

    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
        r1.push_str(", World");
        // let r2 = &mut s;
    }
    println!("Modified Result : {}", s);
}

fn calculate_length(s: &Strign) {
    println!("Borrowed String's len is {}", s.len();
}
