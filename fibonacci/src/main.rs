fn main() {
    let n = 5;
    let f = fibo(n);
    println!("The ans is: {}", f);
}

fn fibo(n: u32) -> u32 {
    if n == 1 || n == 2 {
        n
    } else {
        let mut fib = vec![0, 1];
        for _ in 2..n {
            let next = fib[fib.len() - 1] + fib[fib.len() - 2];
            fib.push(next);
        }
        fib[n as usize - 1]
    }
}
