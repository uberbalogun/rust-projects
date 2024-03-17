fn main() {
    let email = String::from("baloggz@gmail.com");
    let username = String::from("Abayomi");
    let xx = build_user(email, username);
    println!("Name:{} Email: {}  Active: {}  Sing-in count: {}", xx.username, xx.email, xx.active, xx.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
