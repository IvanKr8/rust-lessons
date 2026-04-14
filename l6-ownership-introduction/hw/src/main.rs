// USER PROFILE SYSTEM

#[derive(Clone)]
struct User {
    name: String,
    age: u32,
    nickname: Option<String>,
}

fn consume_user(user: User) {
    println!("User: {}\nAge: {}", user.name, user.age);
    if let Some(n) = user.nickname {
        println!("Nickname: {}", n);
    }
}

fn print_user(user: &User) {
    println!("User: {}\nAge: {}", user.name, user.age);
    if let Some(n) = &user.nickname {
        println!("Nickname: {}", n);
    }
}

fn update_age(user: &mut User, new_age: u32) {
    user.age = new_age;
}

fn rename_user(mut user: User, new_name: String) -> User {
    user.name = new_name;
    user
}

fn main() {
    let mut user1 = User {
        name: String::from("Alice"),
        age: 30,
        nickname: Some(String::from("Ally")),
    };

    print_user(&user1);

    update_age(&mut user1, 31);

    let user1 = rename_user(user1, String::from("Alicia"));

    let clone = user1.clone();

    consume_user(user1);

    println!(" - User clone:");
    print_user(&clone);
}
