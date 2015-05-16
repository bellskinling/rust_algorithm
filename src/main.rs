mod sort;
fn main() {
    let mut arr = [2, 1];
    sort::bubble_sort::<i32>(&mut arr);

    for v in &arr {
        println!("{}", v);
    }
}
