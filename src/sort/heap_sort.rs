// 堆排序,首先构造最大堆, 然后不断地将堆首元素(最大值)和堆尾元素交换, 然后递减
// 堆的大小,再次构造最大堆
pub fn heap_sort<T:Ord>(arr:&mut [T]) {
    build_max_heap(arr);
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        max_heapify(&mut arr[0..i], 0);
    }
}

// 最大堆化, 如果以parent为根节点的子树不满足最大堆的要求
// 那么进行调整.
fn max_heapify<T:Ord>(arr:&mut [T], parent:usize) {
    let left_child = get_left_child(parent);
    let right_child = get_right_child(parent);
    let heap_size = arr.len();
    let mut max_index = parent;
    if left_child < heap_size && arr[left_child] > arr[max_index] {
        max_index = left_child;
    } 
    if right_child < heap_size && arr[right_child] > arr[max_index] {
        max_index = right_child;
    }

    if max_index != parent {
        arr.swap(max_index, parent);
        max_heapify(arr, max_index);
    }
}

fn build_max_heap<T:Ord>(arr:&mut [T]) {
    // 从最大的非叶节点开始建堆
    // 对于基于零的数组, 其最大的非叶节点的序号为 (arr.len() - 1) / 2
    let len = (arr.len() - 1) / 2 + 1;
    for i in (0..len).rev() {
        max_heapify(arr, i);
    }
}

fn get_left_child(parent:usize) ->usize {
    parent * 2 + 1
}

fn get_right_child(parent:usize) ->usize {
    parent * 2 + 2
}
