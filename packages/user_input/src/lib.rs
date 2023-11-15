use std::io;

pub fn get_user_input() -> () {
    let mut input = String::new();

    // Read a single character from the input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Get the first character from the input (if any)
    let user_input = input.trim().chars().last();

    if let Some(input) = user_input {
        println!("{input}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {

    }
}
