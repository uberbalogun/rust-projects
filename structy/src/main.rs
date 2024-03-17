// Define a struct called Product with three fields
struct Product {
    name: String,
    price: f32,
    rating: u8,
}

// Define a method for the Product struct that returns the value for money
impl Product {
    fn value_for_money(&self) -> f32 {
        // Calculate the value for money as the rating divided by the price
        self.rating as f32 / self.price
    }
}

// Create a global constant of the Product struct
const PRODUCT: Product = Product {
    name: String::from("Laptop"),
    price: 999.99,
    rating: 5,
};

fn main() {
    // Call the method on the global constant
    let vfm = PRODUCT.value_for_money();

    // Print the result
    println!("The value for money of {} is {}", PRODUCT.name, vfm);
}
