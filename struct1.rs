
struct User {
    active: bool, 
    username: String,
    email: String,
    sign_in_count: u64
}

struct vec2 (i32, i32);

fn main() {

    fn do_nothing() {
    }

    let u1 = build_user(String::from("me@there.com"));
    let u2 = User {
        email: String::from("daksdjsk"),
        username: String::from("a name"),
        ..u1
    };

    println!("{}", u1.username);
    //println!(u2);

    let v1 = vec2(0, 0);
    println!("{}", v1.0);
}

fn build_user(email: String) -> User {
    let username = String::from("Name");
    User {
        email,
        username, 
        active: true,
        sign_in_count: 1
    }
}



