# rust-test

A simple Rust program that demonstrates the bubble sort algorithm.

## Description

This project implements the bubble sort algorithm in Rust. Bubble sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted.

## Usage

To run the program:

1. Ensure you have Rust installed (see [Rust installation guide](https://www.rust-lang.org/tools/install)).
2. Clone or navigate to the project directory.
3. Run the following command:

```bash
cargo run
```

This will compile and execute the program, displaying the original array and the sorted array.

## Example Output

```
Original array: [64, 34, 25, 12, 22, 11, 90]
Sorted array: [11, 12, 22, 25, 34, 64, 90]
```

## Algorithm Details

The `bubble_sort` function takes a mutable reference to a vector of integers and sorts it in-place using the bubble sort algorithm. It iterates through the array multiple times, swapping adjacent elements if they are out of order, until no more swaps are needed.