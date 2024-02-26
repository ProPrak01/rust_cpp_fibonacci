mod lib;

fn main() {
    let n = 10;
    let fib_n = lib::ffi::fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, fib_n);
}