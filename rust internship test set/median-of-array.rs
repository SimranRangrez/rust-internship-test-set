fn find_median(sorted_array: &[i32]) -> f64 {
    let len = sorted_array.len();
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (sorted_array[mid_left] + sorted_array[mid_right]) as f64 / 2.0
    } else {
        sorted_array[len / 2] as f64
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let median = find_median(&sorted_array);
    println!("The median is: {}", median);
}