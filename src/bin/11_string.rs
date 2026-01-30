fn main() {
    //Initiate string
    let text1 = "Hi".to_string();
    let text2 =String::from("Hi");

    println!("{}",text1);
    println!("{}",text2);

    //Change string
    let mut text3="Hi".to_string();
    text3.push_str(" Tento");
    println!("{}",text3);

    let s1="a";
    let s2="b";
    let s3="c";
    let result = format!("{} {} {}",s1,&s2,&s3);
    println!("{}",result);
}