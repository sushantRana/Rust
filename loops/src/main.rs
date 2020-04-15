fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let b = (10, 20, 30, 40, "50", "Name");

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // for element in b {
    //     println!("the value is: {}", element);
    // }
    
    // cannot loop through tuples
    // let mut index = 0;
    // loop {
    //     println!("{}", b);
    //     index = index + 1;
    // }
}