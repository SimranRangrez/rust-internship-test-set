fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

fn main() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let k = 3;
    if let Some(kth_smallest) = kth_smallest_element(&array, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest);
    } else {
        println!("Invalid value of k");
    }
}