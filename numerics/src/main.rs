//
// hello world in rust ..
//
//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // tuple declaration of different types
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // character
    let c = 'z'; // implicit
    let z : char = 'z'; // with explicit type annotation

    // boolean
    let t = true; // implicit
    let f : bool = false; // explicit

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1

    // remainder
    let remainder = 43 % 5;
} // main
