use std::collections::HashMap;

struct Solution;

impl Solution {
    fn convert_to_int(&self, s: &str) -> i32 {
        let mut roman: HashMap<char, i32> = HashMap::new();
        roman.insert('I', 1);
        roman.insert('V', 5);
        roman.insert('X', 10);
        roman.insert('L', 50);
        roman.insert('C', 100);
        roman.insert('D', 500);
        roman.insert('M', 1000);

        let mut total = 0;
        let mut prev_value = 0;

        for char in s.chars() {
            let current_value = *roman.get(&char).unwrap();
            if current_value > prev_value {
                total += current_value - 2 * prev_value;
            } else {
                total += current_value;
            }
            prev_value = current_value;
        }

        total
    }
}

fn main() {
    let roman_numeral = "III";
    let init = Solution;
    let result = init.convert_to_int(roman_numeral);
    println!("The integer value of {} is: {}", roman_numeral, result);
}

