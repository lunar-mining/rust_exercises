use std::io;
// create a program that converts between C and F
//
// C = (F - 32)*5/9
// F = (9/5*C) + 32

fn main() {
    println!("Enter a temperature to convert it to Celcius or Fahrenheit");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");
}
