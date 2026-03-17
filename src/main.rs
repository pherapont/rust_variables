use std::io;

fn main() {
    println!{"Fibonacci"};
    println!{"Put number for fib: "};

    let mut input = String::new();
    let mut num: u32 = 0;

    io::stdin()
        .read_line(&mut input)
        .expect("Wrong input!");

    num = input.trim().parse().expect("Wrong input");

    let res: u128 = get_fib(num);
    println!("Fibonacci of {num}  = {res}");
}

fn get_fib(n: u32) -> u128 {
    let mut first: u128 = 0;
    let mut second: u128 = 1;
    let mut res: u128 = 0;

    if n == 0 { return 0 }
    else if n == 1 { return 1 }

    let mut i = 2;

    while i <= n {
        res = first + second;
        first = second;
        second = res;
        i = i + 1;
    }
    res
}
