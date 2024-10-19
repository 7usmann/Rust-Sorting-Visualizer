mod sorting_algorithms;

use sorting_algorithms::bubble_sort::bubble_sort;

fn main() {

    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", arr);

    let steps = bubble_sort(&mut arr);

    println!("Sorted array: {:?}", arr);

    for (i, step) in steps.iter().enumerate() {
        println!("Step {}: {:?}", i + 1, step);
    }
}