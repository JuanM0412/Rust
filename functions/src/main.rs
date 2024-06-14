fn nine() -> i32 {
    9
}

fn another_function(x: i32, y: i32) {
    // This is a comment
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = nine();
    println!("The value of x is: {}", x);

    another_function(5, 6);
    
    let y = plus_one(10);
    println!("The value of y is: {}", y);
}