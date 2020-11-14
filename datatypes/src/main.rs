fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}",guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (j,k,l) = tup;

    println!("k is : {}", k);

    let tuple_element = tup.0;

    println!("tuple.1 is : {}", tuple_element);
}
