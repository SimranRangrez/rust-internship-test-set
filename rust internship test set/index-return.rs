fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;

        if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    if low < arr.len() && arr[low] == target {
        Some(low)
    } else {
        None
    }
}

fn main() {
    let sorted_array = vec![1, 2, 2, 3, 4, 4, 4, 5, 6];
    let target_number = 4;

    match first_occurrence_index(&sorted_array, target_number) {
        Some(index) => println!("The first occurrence of {} is at index {}", target_number, index),
        None => println!("{} not found in the array", target_number),
    }
}