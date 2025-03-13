fn fizbzz() {
    let mut inc = 0;
    for i in 1..301{
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
            inc+=1;
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        }
    }
    println!("Total: {}", inc);
}

fn main() {
    println!("Hello, world!");

    fizbzz();

}
