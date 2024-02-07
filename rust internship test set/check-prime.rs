fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let test_number = 17;
    if is_prime(test_number) {
        println!("{} is a prime number", test_number);
    } else {
        println!("{} is not a prime number", test_number);
    }
}