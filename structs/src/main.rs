struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("John"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        email: String::from("doe@example.com"),
        ..user1
    };
    println!(
        "Hello! I'm {}, my email is {}, I'm signed in {}, and active {}",
        user2.username, user2.email, user2.sign_in_count, user2.active
    );
}
