fn main() {
    println!("{}",is_palindrome(1221));
    println!("{}",is_palindrome(123));
}
fn is_palindrome(x:i32) -> bool {
    let str_x = x.to_string();
    let str_invertida: String = str_x.chars().rev().collect();
    return str_x == str_invertida;
}