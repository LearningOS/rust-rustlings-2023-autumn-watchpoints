// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. 
//Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
// 如果您通过，此函数将拒绝生成要打印在姓名标签上的文本
// 它是一个空字符串。 如果能解释一下问题所在就更好了
// 而不是有时返回 `None`。 值得庆幸的是，Rust 有类似的
// 构造可用于表达错误条件的“Result”。 让我们使用
// 它！
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// https://rustwiki.org/zh-CN/std/result/index.html
pub fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.is_empty() {
        // Empty names aren't allowed.
       // None
       Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
