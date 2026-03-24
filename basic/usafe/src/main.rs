fn main() {
    let mut x: i32 = 42;
    let r1 = &x as *const i32;
    let r2 = &mut x as *mut i32;

    unsafe {
        println!("r1 : {}, r2 : {}", *r1, *r2);
        *r2 = 99;
        println!("x = {}", x);
    }
}
