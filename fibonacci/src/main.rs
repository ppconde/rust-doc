use std::io;

// Write the nth fibonacci number
fn main() {
    println!("Insert the number of the fibonacci sequence you want to know: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let n = number.trim().parse().expect("Please type a number");
    println!("The {} fibonacci number is {}", number, fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}
