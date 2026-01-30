fn main() {
    let grade=80;

    if grade >= 80 {
        println!("Grade A")
    }
    else if grade >=60  {
        println!("Grade B")
    }
    else{
        println!("Grade C")
    }

    let number=5;
    let result = if number < 5 {"below 5"} else {"greater than 5"};
    println!("The result is:{}",result)
}