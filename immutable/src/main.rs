//
// hello world in rust ..
//
//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
  let x = 5;
  let x = x + 1;
  {
    let x = x * 2;
    println!("The value of x of the inner scope is: {x}");
  }
  println!("The value of x is: {x}");

  let _guess: u32 = "42".parse().expect("Not a number");
} // main
