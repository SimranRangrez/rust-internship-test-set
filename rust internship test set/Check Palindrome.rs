fn is_palindrome(s: &str) -> bool {
    // Remove non-alphanumeric characters and convert to lowercase
    let cleaned_string: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let cleaned_string_lowercase = cleaned_string.to_lowercase();

    // Check if the cleaned string is equal to its reverse
    cleaned_string_lowercase == cleaned_string_lowercase.chars().rev().collect::<String>()
}

fn main() {
    // Example usage:
    let input_string = "A man, a plan, a canal: Panama";
    let result = is_palindrome(input_string);

    println!("Input string: {}", input_string);
    println!("Is it a palindrome? {}", result);
}