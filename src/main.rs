fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    let num1 = 20;
    let num2 = 22;

    println!("{} + {} = {}", num1, num2, add(num1, num2));
}
