fn greet(name : &str){
    println!("Hi {}",name)
}

fn sum(a:i32,b:i32) -> i32{
    return a+b;
}

fn min(a:i32,b:i32) -> i32{
    return a-b;
}

fn main(){
    greet("Tento");
    println!("Result sum : {}", sum(2, 3));

    let result_min=min(2, 3);
    println!("Result min : {}",result_min)
}