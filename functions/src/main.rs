fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // let func: fn(i32, i32) -> i32 = add;
    let result = add(3, 5);
    println!("Result: {}", result);
}
