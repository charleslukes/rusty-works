use std::io;

fn main() {
    println!("Welcome to Fibonacci calculator");
    println!("Please input number you want to calculate");
    let mut user_requests = String::new();
    io::stdin()
        .read_line(&mut user_requests)
        .expect("Invalid input");
    let user_requests: i64 = user_requests.trim().parse().expect("Must be a number");
    let result = fibonacci(user_requests);
    println!("The fibonacci of {user_requests} is {result}")
}

fn fibonacci(n: i64) -> i64 {
    let mut f_1 = 1;
    let mut f_2 = 1;
    let mut f_next: i64 = 0;

    if n < 3 {
        f_next = 1
    }

    for _ in 3..=n {
        f_next = f_2 + f_1;
        f_1 = f_2;
        f_2 = f_next;
    }

    f_next
}
