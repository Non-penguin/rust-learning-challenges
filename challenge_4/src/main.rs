struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

fn main(){
    let mut user1 = User{
        email: String::from("hoge@example.com"),
        username: String::from("hoge"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("new_hoge@example.com");
    user1.sign_in_count = 2;

    println!("New mail address: {}", user1.email);
}
