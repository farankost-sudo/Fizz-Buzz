
fn main() {
    let number = [3, 5];
    for i in 1..=100 {
        match [i % number[0], i % number[1]] {
            [0, 0] => println!("FizzBuzz"),
            [0, _] => println!("Fizz"),
            [_, 0] => println!("Buzz"),
            _ => println!("{i}"),
        }
    }
}




















