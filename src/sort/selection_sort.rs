pub fn selection_sort<T:Ord + Copy>(arr:&mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut iMin = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[iMin] {
                iMin = j;
            }
        }
        arr.swap(i, iMin);
    }
}
