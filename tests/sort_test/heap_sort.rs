extern crate rust_algorithm;
use rust_algorithm::sort::heap_sort;

#[test]
fn heap_sort_test() {
    let mut arr1 = [1];
    heap_sort(&mut arr1);
    assert_eq!(arr1, [1]);


    let mut arr2 = [3, 1];
    heap_sort(&mut arr2);
    assert_eq!(arr2, [1, 3]);


    let mut arr3 = [3, 1, 2, 10];
    heap_sort(&mut arr3);
    assert_eq!(arr3, [1, 2, 3, 10]);


    let mut arr4 = [0, -13, 12, 3, 1, 2, 10];
    heap_sort(&mut arr4);
    assert_eq!(arr4, [-13, 0, 1, 2, 3, 10, 12]);
}
