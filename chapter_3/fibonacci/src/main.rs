use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Enter a number");

    let fibonacci_number =
        get_fibonacci_number(n.trim().parse().expect("Ensure a number is used."));
    println!("{fibonacci_number}");
}

fn get_fibonacci_number(n: u64) -> u64 {
    let mut x__: u64 = 0;
    let mut x_: u64 = 1;
    let mut x: u64 = x_;
    if n == 0 {
        return x__;
    }
    for _count in 1..n {
        x = x_ + x__;
        x__ = x_;
        x_ = x;
    }
    return x;
}
