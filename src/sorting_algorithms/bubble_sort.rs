pub fn bubble_sort(arr: &mut [i32]) -> Vec<Vec<i32>> {
    let mut steps = Vec::new();
    let len = arr.len();
    let mut swap = true;

    while swap {
        swap = false;
        for i in 0..len - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swap = true; 
            }
        }
        steps.push(arr.to_vec());
    }
    steps
}
