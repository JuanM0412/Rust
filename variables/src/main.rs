fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    let a = 2.0;
    println!("The value of a is: {}", a);
    let b: f32 = 3.0;
    println!("The value of b is: {}", b);

    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);
    let product = 4 * 30;
    println!("The value of product is: {}", product);
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let t = true;
    println!("The value of t is: {}", t);
    let f: bool = false;
    println!("The value of f is: {}", f);

    let z = 'z';
    println!("The value of z is: {}", z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t1, t2, t3) = tup;
    println!("The value of t1 is: {}", t1);
    println!("The value of t2 is: {}", t2);
    println!("The value of t3 is: {}", t3);
    let five_hundred = tup.0;
    println!("The value of tup.0 is: {}", five_hundred);

    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
}