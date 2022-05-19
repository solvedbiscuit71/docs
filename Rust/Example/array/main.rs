use std::mem;

fn sort_array(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..i {
            if arr[i] < arr[j] {
                (arr[i], arr[j]) = (arr[j], arr[i]);
            }
        }
    }
}

/*
 * Issue:
 * 1. the size of the array is fixed!
 * 2. the size should be known at compile time
 */

fn main() {
    const SIZE: usize = 6; // NOTE: const NAME: type-annotation = value;
    let mut arr: [i32; SIZE] = [1, 5, 3, 4, 2, 0];
    println!(
        "i32 -> 32bits -> 4bytes * 6 = {}bytes",
        mem::size_of_val(&arr)
    );

    println!("{:?}", arr);
    sort_array(&mut arr[0..4]);
    println!("{:?}", arr);
}
