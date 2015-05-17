pub fn quick_sort<T:Ord + Copy>(arr:&mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, len);
}

fn _quick_sort<T:Ord + Copy>(arr:&mut [T], begin:usize, end:usize) {
    if end == 0 || begin >= end - 1 {
        return;
    }
    let pivot = partition(arr, begin, end);
    _quick_sort(arr, begin, pivot);
    _quick_sort(arr, pivot + 1, end);
}

fn partition<T:Ord + Copy>(arr:&mut [T], begin:usize, end:usize) -> usize {

    if end == 0 || begin >= end -1 {
        return begin;
    }
    let pivot = arr[end - 1];

    let mut i = begin as i32 - 1;
    for j in begin..end {
        if arr[j] < pivot {
            i += 1;
            arr.swap(i as usize, j);
        }
    }

    arr.swap( (i + 1) as usize, end - 1);
    return (i + 1) as usize
}
