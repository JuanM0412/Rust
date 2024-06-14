use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius* (9.0 / 5.0) + 32.0
}

fn fibonacci_sequence(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_sequence(n - 1) + fibonacci_sequence(n - 2),
    }
}

fn main() {
    loop {
        println!("Choose an option to go: \n1. Fahrenheit to Celsius. \n2. Celsius to Fahrenheit. \n3. Generate the Fibonacci sequence.");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if option == 1 {
            println!("Enter the temperature in Fahrenheit:");
            let mut fahrenheit = String::new();

            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read line");

            let fahrenheit: f64 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            };

            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{} °F is equal to {} °C", fahrenheit, celsius);
        } else if option == 2 {
            println!("Enter the temperature in Celsius:");
            let mut celsius = String::new();

            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read line");

            let celsius: f64 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            };

            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("{} °C is equal to {} °F", celsius, fahrenheit);
        } else if option == 3 {
            println!("Wich number do you want to find in the Fibonacci sequence? (n position):");
            let mut n = String::new();

            io::stdin()
                .read_line(&mut n)
                .expect("Failed to read line");

            let n: i64 = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            };

            println!("The number n of the Fibonacci sequence is: {}", fibonacci_sequence(n));
        } else {
            break;
        }
    }
}