pub fn selection_sort<T:Ord + Copy>(arr:&mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut i_min = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[i_min] {
                i_min = j;
            }
        }
        arr.swap(i, i_min);
    }
}
