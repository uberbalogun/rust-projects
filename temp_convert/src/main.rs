use std::io;

fn main() {
    loop {
        println!("Please input a temperature value (number):");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // Convert the user input to a floating-point number
        let user_input: f64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        // Ask the user if the input is in Celsius or in Fahrenheit
        println!("Is the number inputted in Celsius? (y/n)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read line");
        if response.trim().to_lowercase() == "y" {
            let f_result = to_fahrenheit(user_input);
            println!("{:?}°F", f_result); // Added curly braces around f_result

        } else if response.trim().to_lowercase() == "n" {
            let c_result = to_celsius(user_input); // Corrected function name to_celsius
            println!("{:?}°C", c_result); // Added curly braces around c_result

        } else {
            break;
        }
    }
}

fn to_fahrenheit(c: f64) -> f64 {
    (9.0 / 5.0 * c) + 32.0
}

fn to_celsius(f: f64) -> f64 {
    5.0 / 9.0 * (f - 32.0)
}
