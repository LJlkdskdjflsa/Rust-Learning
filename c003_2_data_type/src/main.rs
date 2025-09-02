fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    let x = 2.0; // f64, default
    println!("x: {}", x);

    let y: f32 = 3.0; // f32
    println!("y: {}", y);

    println!("Hello, world!");

    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("truncated: {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation

    // === Char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // ==== Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // ==== The Array Type
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
