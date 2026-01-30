fn main(){
    let mut numbers = [1,2,3,4,5];

    for num in numbers{
        println!("number: {}",num)
    }

    numbers[0]=10;
    println!("Number ke 1: {}",numbers[0]);
    println!("Size of array: {}",numbers.len());
    println!("{:?}",numbers);
}