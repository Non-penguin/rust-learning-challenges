type CustimResult = Result<i32, String>;

fn main(){
    let some_result: CustimResult = Ok(42);
    let some_result_err: CustimResult = Err(String::from("An error occurred"));

    match some_result {
        Ok(num) => {
            println!("Success with value {}.", num);
        }
        Err(msg) => {
            println!("Failure with message: {}.", msg);
        }
    }

    match some_result_err {
        Ok(num) => {
            println!("Success with value {}.", num);
        }
        Err(msg) => {
            println!("Failure with message: {}.", msg);
        }
    }
}
