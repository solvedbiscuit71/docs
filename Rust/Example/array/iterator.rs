/*
 * a collection can be converted to an iterator.
 * 1. iter() -> get reference
 * 2. into_iter() -> copies the value
 *
 * then, after performing some transformation it's converted back to
 * collection by `collect()` method
 */

fn main() {
    let arr: [i32; 7] = [0, -3, 2, 9, -10, 8, 1];
    let arr = arr.iter();

    let mut positive_no = arr.filter(|&no| no.is_positive());
    let mut arr = Vec::new();

    while let Some(num) = positive_no.next() {
        arr.push(num);
    }
    println!("Array: {:?}", arr);

    let mut arr: Vec<i32> = arr.iter().map(|&no| no * 2).collect();
    arr.sort();
    arr.iter()
        .enumerate()
        .for_each(|(i, &no)| println!("{}: {}", i, no));

    let sum = arr.iter().fold(0, |sum, &no| sum + no);
    /*
     * reduce() is a special case of fold where, the accumulator is the first element.
     */

    println!("Sum of all element is {}", sum);
}
