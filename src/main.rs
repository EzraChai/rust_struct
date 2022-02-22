struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        username: String::from("Chloe Gan"),
        email: String::from("chloegan.cg@###.###"),
        sign_in_count: 34,
        active: true,
    };
    let user2 = User {
        username: String::from("Ezra Chai"),
        email: String::from("juanzhe2@####.###"),
        ..user1
    };

    println!("User 1's Username: {}", user1.username);
    println!("User 2's Username: {}", user2.username);

    struct Color(i32, i32, i32);
    let white = Color(255, 255, 255);
    println!("{} ({}, {}, {})", "white", white.0, white.1, white.2);
}
