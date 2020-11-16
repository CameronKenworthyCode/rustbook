fn main() {
    println!("Hello, world!");
    let y = 6;
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("Another function. X is {}, y is {}", x, y);
}