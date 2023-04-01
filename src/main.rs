mod calculation {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    pub fn divide(a: f64, b: f64) -> f64 {
        a / b
    }
}

fn main() {
    println!("Enter the first number:");
    let mut first_num = String::new();
    std::io::stdin()
        .read_line(&mut first_num)
        .expect("Failed to read line");

    let first_num: f64 = first_num
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Enter the second number:");
    let mut second_num = String::new();
    std::io::stdin()
        .read_line(&mut second_num)
        .expect("Failed to read line");

    let second_num: f64 = second_num
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    std::io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let operation = operation.trim();

    let result = match operation {
        "+" => calculation::add(first_num, second_num),
        "-" => calculation::subtract(first_num, second_num),
        "*" => calculation::multiply(first_num, second_num),
        "/" => calculation::divide(first_num, second_num),
        _ => panic!("Invalid operation"),
    };

    println!("Result: {}", result);
}
