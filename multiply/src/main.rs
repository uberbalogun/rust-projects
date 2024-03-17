use std::io;

fn main() {
    loop {
        println!("Please input the first number:");

        let mut user_input_1 = String::new();

        io::stdin()
            .read_line(&mut user_input_1)
            .expect("Failed to read line");

        let user_input_1: u32 = match user_input_1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        loop {
            println!("Please input the second number:");

            let mut user_input_2 = String::new();

            io::stdin()
                .read_line(&mut user_input_2)
                .expect("Failed to read line");

            let user_input_2: u32 = match user_input_2.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    continue;
                }
            };

            let total = user_input_1 * user_input_2;

            println!("The result of multiplying your input is: {}", total);

            // Ask if the user wants to continue
            println!("Do you want to input more numbers? (y/n)");
            let mut response = String::new();
            io::stdin().read_line(&mut response).expect("Failed to read line");
            if response.trim().to_lowercase() != "y" {
                break;
            }
        }
    }
}
