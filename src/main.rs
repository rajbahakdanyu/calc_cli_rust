fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

fn muntiply(x: i32, y: i32) -> i32 {
    return x * y;
}

fn divide(x: i32, y: i32) -> i32 {
    return x / y;
}

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut action = String::new();

    loop {
        println!("First number:");
        std::io::stdin().read_line(&mut num1);
        println!("Second number:");
        std::io::stdin().read_line(&mut num2);
        println!("Select operation Add(1), Subtract(2), Muntiply(3), Divide(4) or Exit(0)");
        std::io::stdin().read_line(&mut action);
    }
}
