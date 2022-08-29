/// This is a nonsense function
///
/// # But there's more
///
/// it so happens that you can call this function!
///
/// ```rust
/// wat();
/// ```
///
/// But what does it do? 🤷🏻‍♂️
///
pub fn wat() {
    let stuff = vec![1, 2, 3];
    let boxed = Box::new(stuff);
    let v2 = boxed.iter().rev().map(|x| x + 1_i32).map(|n| n + 10_i32);
    for item in v2 {
        println!("found {}", item);
    }
}
