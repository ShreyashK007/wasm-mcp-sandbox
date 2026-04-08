use std::env;

// OPTIMIZED: Iterative Fibonacci (O(n) time complexity)
fn fibonacci(n: u32) -> u32 {
    if n <= 1 { return n; }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return; }
    
    let target_str = &args[1];
    let num: u32 = target_str.parse().unwrap_or(0);

    let result = fibonacci(num);
    
    println!("SUCCESS! The WebAssembly Sandbox calculated the Fibonacci number for {} is: {}", num, result);
}