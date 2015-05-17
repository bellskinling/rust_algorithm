use super::super::tools::GetMax;
pub fn merge_sort<T:Ord + Clone + Copy + GetMax>(arr:&mut [T]) {
    let length = arr.len();
    _merge_sort(arr, 0, length);
}

fn _merge_sort<T:Ord + Clone + Copy + GetMax>(arr:&mut [T], left:usize, right:usize) {
    // 当只有一个元素时,返回
    if left < right - 1{
        let middle = (left + right) / 2;
        _merge_sort(arr, left, middle);
        _merge_sort(arr, middle, right);
        merge(arr, left, middle, right);
    }
}

// 将[left, middle)和 [middle, right)按从小到大的顺序合并为一个数组, 注意区间是
// 左闭右开
fn merge<T:Ord + Clone + Copy + GetMax>(arr:&mut [T], left:usize, middle:usize, right:usize) {
    let mut arr1 = arr[left..middle].to_vec();
    let mut arr2 = arr[middle..right].to_vec();
    // 在arr1和arr2的末尾添加一个哨兵值,其值大小为T::max_value(),
    // 这样在循环中就不需要再对数组的边界条件进行判断了
    arr1.push(T::max_value());
    arr2.push(T::max_value());

    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if arr1[i] < arr2[j] {
            arr[k] = arr1[i];
            i += 1;
        } else {
            arr[k] = arr2[j];
            j += 1;
        }
    }
}
