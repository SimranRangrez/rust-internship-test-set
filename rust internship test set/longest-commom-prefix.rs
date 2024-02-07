fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_str = strings[0];
    let mut common_prefix = String::new();

    for (i, char) in first_str.chars().enumerate() {
        if strings.iter().all(|s| s.chars().nth(i) == Some(char)) {
            common_prefix.push(char);
        } else {
            break;
        }
    }

    common_prefix
}

fn main() {
    let string_set = vec!["flower", "flow", "flight"];
    let prefix = longest_common_prefix(&string_set);
    println!("The longest common prefix is: {}", prefix);
}