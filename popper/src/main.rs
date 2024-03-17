use std::io;

fn main(){
    
    loop {
        println!("Please input the first number:");
   
        let mut user_input_1 = String::new();

        io::stdin()
            .read_line(&mut user_input_1)
            .expect("Failed to read line");

        let user_input_1: u32 = match user_input_1.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
        };

        loop {
            println!("Please input the second number:");
    
            let mut user_input_2 = String::new();
    
            io::stdin()
                .read_line(&mut user_input_2)
                .expect("Failed to read line:");

            let user_input_2: u32 = match user_input_2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        
        }; 
            
        let total = multiply(user_input_1, user_input_2);

        println!("The result of multiplying your input is:{}", total); 
            break;
        }
        break;
    }
}

fn multiply(num1: u32, num2: u32)->u32{
    num1 * num2
}