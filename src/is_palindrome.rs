//checks if an integer is a palindrome

pub fn is_palindrome(int: i32) -> bool {

    let mut s = String::from(int.to_string());
    let mut revert = String::from(""); 

    while let Some(top) = s.pop() {
        revert.push(top);
    }
    println!("Og: {}, revert: {}", String::from(int.to_string()), revert);
    revert == String::from(int.to_string())
}