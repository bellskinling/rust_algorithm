//声明类型T实现了Ord和Copy Trait,如果不声明Copy这个Trait,
//那么语句"let key = arr[i]"将被编译器认为是移动语义
pub fn insertion_sort<T:Ord + Copy>(arr:&mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && key < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}
