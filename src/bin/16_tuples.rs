fn get_user() -> (String, i32) {
    return (String::from("Tento"), 21);
}

fn main() {
    let person = ("Tento", 21, true);
    println!("Name:{}", person.0);
    println!("Age:{}", person.1);
    println!("Is true:{}", person.2);

    println!();

    let (name, age, person) = person;
    println!("Name:{}", name);
    println!("Age:{}", age);
    println!("Is true:{}", person);

    let user = get_user();
    println!("My name is {} and my age is {}", user.0, user.1);
}
