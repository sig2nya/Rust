
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
}
