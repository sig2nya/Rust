trait Animal {
    fn sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) { println!("Wal Wal"); }
}

impl Animal for Cat {
    fn sound(&self) { println!("Meow Meow"); }
}

fn main() {
    let x = 5; // Stack
    let y = Box::new(5); // Heap
    println!("{}", *y); // y is a pointer

    /*
     * let y = Box::new(5);
     * --------------------
     * int *y = malloc(sizeof(int));
     * *y = 5;
     * printf("%d\n", *y);
     * free(y);
     *
     * */

    let b = Box::new(10);
    println!("b = {}", b);

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for a in animals {
        a.sound();
    }
}
