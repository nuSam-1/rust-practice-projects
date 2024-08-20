pub fn reverse(input: &String) -> String {
   input.chars().rev().collect() 
}

pub fn inspect(input: &String, digits: bool) -> (i32, String){
    if !digits {
        return (input.len() as i32, String::from("char"));
    }

    return (inspect_numbers(input), String::from("digit"));
}

fn inspect_numbers(input: &String) -> i32 {
    let mut count = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }

    count
}

pub fn is_palindrome(input: &String) -> bool {
    let reversed_input: String = input.chars().rev().collect();

    if *input == reversed_input {
        true
    } else {
        false
    }
}
