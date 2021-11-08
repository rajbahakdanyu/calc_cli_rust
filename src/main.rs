fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

fn muntiply(x: i32, y: i32) -> i32 {
    return x * y;
}

fn main() {
    let num1 = 20;
    let num2 = 22;

    println!("{} + {} = {}", num1, num2, add(num1, num2));
    println!("{} - {} = {}", num1, num2, subtract(num1, num2));
    println!("{} - {} = {}", num1, num2, muntiply(num1, num2));
}
