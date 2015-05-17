extern crate rust_algorithm;
use rust_algorithm::sort::quick_sort;

#[test]
fn quick_sort_test() {
    let mut arr1 = [1];
    quick_sort(&mut arr1);
    assert_eq!(arr1, [1]);


    let mut arr2 = [3, 1];
    quick_sort(&mut arr2);
    assert_eq!(arr2, [1, 3]);


    let mut arr3 = [10, 1, 2, 3];
    quick_sort(&mut arr3);
    assert_eq!(arr3, [1, 2, 3, 10]);


    let mut arr4 = [0, -13, 12, 3, 1, 2, 10];
    quick_sort(&mut arr4);
    assert_eq!(arr4, [-13, 0, 1, 2, 3, 10, 12]);



    let mut arr5 = [134, 2, 513, -2, 34, 29, 0, 1, -3, 100, 13, 61435, 0, -13, 12, 3, 1, 2, 10];
    quick_sort(&mut arr5);
    assert_eq!(arr5, [-13, -3, -2, 0, 0, 1, 1, 2, 2, 3, 10, 12, 13, 29, 34, 100, 134, 513, 61435]);
}
