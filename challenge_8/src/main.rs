fn get_result(success: bool) -> Result<i32, String> {
    if success {
        Ok(100)
        //error("Simulated error"))
        //もしエラーでテストしたかったら、上の行をアンコメントして、下の行をコメントアウトして！
    } else {
        Err(String::from("An error occurred"))
    }
}

fn main() {
    let result = get_result(true);

    let number: i32 = result.expect("Failed to get the number");
    println!("The number is: {}", number); 
}