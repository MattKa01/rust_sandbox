mod two_sum;
mod is_palindrome;
mod first_last;

fn main() {
    let res: Vec<i32> = two_sum::run(vec![1,2,3,4,5], 8);
    println!("Result: {:?}", res);

    println!("Result Palindrome: {}", is_palindrome::is_palindrome(121));

    let fl: Vec<i32> = first_last::first_last(vec![1,2,3,4,5,1,3,4,1],3);

    println!("Result First Last: {:?}", fl);

}

