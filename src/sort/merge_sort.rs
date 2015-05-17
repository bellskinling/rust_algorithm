use std::cmp;
pub fn merge_sort<T:Ord + Clone + Copy>(arr:&mut [T]) {
    let length = arr.len();
    _merge_sort(arr, 0, length);
}

fn _merge_sort<T:Ord + Clone + Copy>(arr:&mut [T], left:usize, right:usize) {
    // 当只有一个元素时,返回
    if left < right - 1{
        let middle = (left + right) / 2;
        _merge_sort(arr, left, middle);
        _merge_sort(arr, middle, right);
        merge(arr, left, middle, right);
    }
}

fn copy<T:Copy>(dist:&mut[T], src:&[T]) {
    let len = cmp::min(dist.len(), src.len());
    for i in 0..len {
        dist[i] = src[i];
    }
}

// 将[left, middle)和 [middle, right)按从小到大的顺序合并为一个数组, 注意区间是
// 左闭右开
fn merge<T:Ord + Clone + Copy>(arr:&mut [T], left:usize, middle:usize, right:usize) {
    let arr1:Vec<T> = arr[left..middle].to_vec();
    let arr2:Vec<T> = arr[middle..right].to_vec();

    // 如果可以在arr1和arr2的最末尾再添加一个哨兵元素,其值是T::max_value(), 那么
    // 合并的过程就不需要这么多的判断了

    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if arr1[i] < arr2[j] {
            arr[k] = arr1[i];
            i += 1;
            if i == arr1.len() {
                if k + 1 < right {
                    copy(&mut arr[k + 1..right], &arr2[j..]);
                    break;
                }
            }
        } else {
            arr[k] = arr2[j];
            j += 1;
            if j == arr2.len() {
                if k + 1 < right {
                    copy(&mut arr[k + 1..right], &arr1[i..]);
                    break;
                }
            }
        }
    }
}
