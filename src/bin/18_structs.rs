struct Person {
    name: String,
    age: i32,
    can_vote: bool,
}

fn main() {
    let user = Person {
        name: String::from("Tento"),
        age: 21,
        can_vote: true,
    };

    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Can Vote: {}", user.can_vote);
}
