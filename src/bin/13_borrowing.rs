fn main(){
    let s1=String::from("Hi Tento");
    let s2 = &s1;

    // s2 borrowing value from s1
    println!("{}",s1);
    println!("{}",s2);

    //mutable borrowing
    let mut s3=String::from("Hello ");
    let ref_s3=&mut s3;
    ref_s3.push_str("World");
    println!("{}",ref_s3);
}