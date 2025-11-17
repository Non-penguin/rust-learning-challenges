fn fizzbuzz_check(n: i32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "Fizzbuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => n.to_string(),
    }
}

fn main() {
    println!("--- FizzBuzz Check ---");
    for i in 1..=100 {
        println!("{}", fizzbuzz_check(i));
    }
}
