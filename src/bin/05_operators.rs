fn main() {
    let add = 2 + 3;
    let min = 2 - 3;
    let mul = 2 * 3;
    let div = 2 / 3;
    let rem = 2 % 3;

    println!("Add: {}", add);
    println!("Add: {}", min);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Mod: {}", rem);

    let mut x = 10;

    x += 2;
    println!("after add 2: {}", x);

    x -= 2;
    println!("after min 2: {}", x);

    x *= 2;
    println!("after mul 2: {}", x);

    x /= 2;
    println!("after div 2: {}", x);

    x %= 2;
    println!("after mod 2: {}", x);

    let a = 2;
    let b = 3;
    println!("a == b:{}", a == b);
    println!("a != b:{}", a != b);
    println!("a > b:{}", a > b);
    println!("a < b:{}", a < b);
}
