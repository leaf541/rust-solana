use std::collections::HashMap;

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

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let comp = target - num;
        if let Some(&index) = map.get(&comp) {
            return vec![index as i32, i as i32];
        }
        map.insert(num , i);
    }
    vec![]
}

fn main() {
    println!("Hello, world!");

    fizbzz();

    println!("{:?}", two_sum(vec![2, 3, 4, 5,], 9));


}
