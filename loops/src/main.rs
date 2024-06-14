fn main() {
    loop {
        println!("again!");

        break;
    }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("While loop");
    while index < 5 {
        println!("The value is: {}", arr[index]);

        index = index + 1;
    }

    println!("For loop");
    for element in arr.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
