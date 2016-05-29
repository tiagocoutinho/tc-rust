fn fib(num: u64) -> u64 {
    if num < 2 { 1 } else { fib(num - 2) + fib(num - 1) }
}

fn main() {
    println!("fib(38) = {}", fib(38));
}
