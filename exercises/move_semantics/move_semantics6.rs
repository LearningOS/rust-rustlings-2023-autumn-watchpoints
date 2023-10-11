// move_semantics6.rs
//
// You can't change anything except adding or removing references.
// 除了添加或删除引用之外，您无法更改任何内容。
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.
// 
fn main() {
    // class string stack{ptr--->head}
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
// 这些 & 符号表示引用，它们允许您引用某个值而无需获得该值的所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership 负责释放
//https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html#%E5%8F%AF%E5%8F%98%E5%BC%95%E7%94%A8
// java c++ 引用传递， golang 引用传递 
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
