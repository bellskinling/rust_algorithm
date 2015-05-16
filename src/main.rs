mod sort;
fn main() {
    let mut arr = [2, 1];
    sort::bubble_sort(&mut arr);

    for v in &arr {
        println!("{}", v);
    }
}
