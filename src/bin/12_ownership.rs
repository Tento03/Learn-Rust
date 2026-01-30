fn main() {
    let a=2;
    let b=a;
    let c=b.clone();

    println!("a: {}",a);
    println!("b: {}",b);
    println!("c: {}",c);

    let s1=String::from("Hi Tento");
    let s2=s1;

    //The ownership of value s1 is moved to s2
    // println!("{}",s1); 
    println!("{}",s2);
    
}