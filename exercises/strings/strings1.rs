// strings1.rs
//
// Make me compile without changing the function signature!
// 我在不更改函数签名的情况下编译！
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}
// String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
fn current_favorite_color() -> String {
    "blue".to_string()
}
