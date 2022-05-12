pub fn sort_vector(arr: &mut [i32]) {
    let len = arr.len();
    for cur in 0..len {
        for i in 0..cur {
            if arr[cur] < arr[i] {
                (arr[cur], arr[i]) = (arr[i], arr[cur]);
            }
        }
    }
}

/*
    Here, #[test] makes this a test function
    which can be used by `cargo test`
*/

#[test]
fn unit_test() {
    let mut arr1 = vec![6, 20, 48, 24, 9];
    let arr1_sort = vec![6, 9, 20, 24, 48];

    sort_vector(&mut arr1);
    assert_eq!(arr1, arr1_sort);

    /*
        Macro for Testing.
        assert!() -> checks for true
        assert_eq!() -> checks if equal
        assert_nq!() -> checks if not equal
    */
}
