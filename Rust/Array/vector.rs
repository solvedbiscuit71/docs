fn sort_array(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..i {
            if arr[i] < arr[j] {
                (arr[i], arr[j]) = (arr[j], arr[i]);
            }
        }
    }
}

fn pretty_print(arr: &[i32]) {
    for (i, item) in arr.iter().enumerate() {
        println!("{}: {}", i, item);
    }
}

fn power_2(arr: &mut [i32]) {
    for i in arr.iter_mut() {
        *i *= *i;
    }
}

/*
 * To solve the issue with array we have, Vec<T>
 * 1. it is growable
 */

fn main() {
    let mut arr = vec![1, 5, 3, 8, 20, 5];

    power_2(&mut arr);
    pretty_print(&arr);
    println!("Sorting...");
    sort_array(&mut arr);
    pretty_print(&arr);
}
