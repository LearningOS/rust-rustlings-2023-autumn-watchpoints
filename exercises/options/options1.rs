// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.


// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
// 这个函数返回冰箱中还剩下多少冰淇淋。
// 如果在晚上10点之前，还剩下5块。但是在10点，有人把它们都吃掉了，所以就不再剩下了。
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    //  这里不用hash存储吗？
    //0..=21 表示一个范围，包含以下整数：0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21。
    match time_of_day {
      0..=21 => Some(5),
      22..=23 => Some(0),
      _ => None,
    }

}//
// https://medium.com/@verbruggenjesse/getting-started-with-rust-using-rustlings-part-11-options-eb4973596594

// In Enum Definitions https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap(), 5);
    }
}
//https://doc.rust-lang.org/std/option/

// https://rustwiki.org/zh-CN/rust-by-example/error/option_unwrap.html
// Some(T)：找到一个属于 T 类型的元素 None：找不到相应元素
//unwrap 隐式地处理。隐式处理要么返回 Some 内部的元素，要么就 panic。
// option_unwrap.html
// https://rustwiki.org/zh-CN/rust-by-example/error/option_unwrap.html