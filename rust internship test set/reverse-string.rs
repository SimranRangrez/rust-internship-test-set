fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let original_string = "Hello, World!";
    let reversed_string = reverse_string(original_string);
    
    println!("Original: {}", original_string);
    println!("Reversed: {}", reversed_string);
}