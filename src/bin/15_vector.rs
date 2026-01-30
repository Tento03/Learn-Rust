fn main(){
    let mut fruits = vec!["apple","orange"];
    println!("{:?}",fruits);

    fruits.push("mangoo");
    println!("{:?}",fruits);

    fruits.pop();
    println!("{:?}",fruits);

    fruits.insert(2, "pineapple");
    println!("{:?}",fruits);

    fruits.remove(2);
    println!("{:?}",fruits);

    println!("{}",fruits.len());

    for fruit in &fruits  {
        println!("Fruit: {}",fruit)
    }
}