use std::io;

fn main() {
    loop {
        println!("Enter a temperature to convert it to Celcius or Fahrenheit");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line.");

        let temp: u32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You entered {}:", temp);

        if temp < 32 {
            let f = convert_to_f(temp);
            println!("Temperature in Fahrenheit is: {}", f);
        } else {
            let c = convert_to_c(temp);
            println!("Temperature in Celcius is: {}", c);
        }
    }
}
fn convert_to_f(x: u32) -> u32 {
    ((x/5)*9)+32 
}  

fn convert_to_c(x: u32) -> u32 {
    ((x - 32) * 5) / 9
}


