// BubbleSort sorts a vector of integers using the bubble sort algorithm
fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                // Swap adjacent elements
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];

    println!("Original array: {:?}", numbers);

    bubble_sort(&mut numbers);

    println!("Sorted array: {:?}", numbers);
}