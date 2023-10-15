// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
//capitalize_first("hello"), "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    //input.chars() 返回一个字符迭代器，使我们能够逐个访问输入字符串的字符。
    // 返回一个字符 怎么变成字符串？
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// Some(first) => first.to_uppercase().to_string() + c.as_str(),: 如果迭代器返回Some(first)，也就是输入字符串非空且有至少一个字符，那么它会执行以下操作：

// first.to_uppercase()：将第一个字符first转换为大写形式。
// .to_string()：将大写的字符转换为一个新的字符串。
// + c.as_str()：将这个新的大写字符与剩余的字符拼接起来，c.as_str()将剩余字符转换为字符串并连接在一起。
// what is first
//what is the funciton 它的目的是将输入字符串的第一个字符转换为大写字毅，然后返回一个新的字符串。

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //vec![]
    let v2 = words.iter().map(|x| capitalize_first(x)).collect();
    v2

    //let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}


// 这是一个Rust函数，名为`capitalize_words_vector`，它接受一个字符串切片的引用（`&[&str]`），并返回一个包含经过首字母大写化处理后的字符串的`Vec<String>`。

// 让我们逐行解释这个函数的代码：

// 1. `pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {`: 这一行定义了一个公共（`pub`）函数，名称为`capitalize_words_vector`，它接受一个字符串切片的引用`words`作为参数，并将返回一个新的`Vec<String>`，即字符串的向量。

// 2. `words.iter()`: 这一行使用`.iter()`方法创建了一个迭代器，用于遍历输入的字符串切片`words`中的每个元素。

// 3. `.map(|x| capitalize_first(x))`: 在这一行，`.map()`方法将对每个元素应用一个闭包函数。这个闭包函数接受一个字符串引用`x`，然后调用之前定义的`capitalize_first`函数来将字符串的第一个字符大写化，最终返回一个经过处理的字符串。

// 4. `.collect()`: `.collect()`方法将迭代器中的所有处理后的字符串收集到一个新的`Vec<String>`中，并将这个`Vec`作为函数的返回值。

// 综合来看，这个函数的目的是接受一个字符串切片，将其中的每个字符串的首字母大写化，并将处理后的字符串存储在一个新的向量中。这对于批量处理多个单词或短语，并将它们的首字母大写化，例如处理标题或名字列表等情况非常有用。
// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|x| capitalize_first(x)).collect()
}

//01 遍历 wrods 内容放到 string

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
