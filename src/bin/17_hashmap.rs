use std::collections::HashMap;

fn main() {
    let mut capital_cities = HashMap::new();

    capital_cities.insert("Indo", "Jakarta");
    capital_cities.insert("England", "London");

    println!("{:?}", capital_cities);

    //Access values
    if let Some(city) = capital_cities.get("Indo") {
        println!("The capital of Indo is {}", city);
    } else {
        println!("Indo is not in the map");
    }

    //Update values
    capital_cities.insert("Malaysia", "Malaydesh");
    capital_cities.insert("Malaysia", "Kuala Lumpur");
    println!("{:?}", capital_cities);

    //Remove values
    capital_cities.remove("England");
    println!("{:?}", capital_cities);

    for (country, city) in &capital_cities {
        println!("The capital of {} is {}", country, city);
    }
}
