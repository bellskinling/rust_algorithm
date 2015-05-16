pub fn bubble_sort(arr:&mut [i32]) {
    let mut length = arr.len();
    while length > 1 {
        let mut i = 1;
        //常规的冒泡排序每次内层循环后,length减一,而没有考虑到数组
        //是一个大部分有序的数组.
        //然而如果我们在每次内层循环判断成功后,记录下当前的i的值,
        //这样如果数组是大部分有序的话,那么可以大幅度地缩减length的值,
        //因此可以减小不必要的外层循环
        let mut new_length = 0;
        while i < length {
            if arr[i] < arr[i - 1] {
                arr.swap(i, i - 1);
                new_length = i;
            }
            i += 1;
        }
        length = new_length;
    }
}
