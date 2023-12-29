use std::io;

// Convert temperatures between Fahrenheit and Celsius
fn main() {
    println!("Pick either Celsius or Fahrenheit to convert to the other.");
    println!("1. Celsius");
    println!("2. Fahrenheit");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice");

    choice = choice.trim().to_string();

    if (choice == "1") || (choice == "2") {
        println!("Enter the temperature to convert: ");
    } else {
        println!("Invalid choice.");
        return;
    }

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature.");
            return;
        }
    };

    if choice == "1" {
        println!(
            "{}°C is {}°F",
            temperature,
            convert_to_fahrenheit(temperature)
        );
    } else {
        println!("{}°F is {}°C", temperature, convert_to_celsius(temperature));
    }
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
