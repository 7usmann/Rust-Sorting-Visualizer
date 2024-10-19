pub fn bubble_sort(arr: &mut [i32]) -> Vec<Vec<i32>> {
    let mut steps = Vec::new(); // Store all steps of the sorting process

    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1); // Perform the swap
                steps.push(arr.to_vec()); // Capture the array state after every swap
            }
        }
    }

    steps // Return all steps for visualization
}
