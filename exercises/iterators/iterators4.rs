// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    //要计算给定数字的阶乘而不使用递归、循环或额外变量

    // For an extra challenge, don't use:
    // - recursion
    // 不使用递归
    // Execute `rustlings hint iterators4` for hints.

    //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold

    //https://www.cuemath.com/numbers/factorial/

    (1..=num).fold(1, |acc, x| acc * x)

    //let sum = a.iter().fold(0, |acc, x| acc + x);
    //olds every element into an accumulator by applying an operation, returning the final result.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
