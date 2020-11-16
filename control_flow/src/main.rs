fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("{}!", counter);

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    for element in a.iter() {
        println!("The Vaule is : {}", element);
    }
}