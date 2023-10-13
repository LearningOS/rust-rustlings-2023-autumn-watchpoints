// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.
// 假设我们正在编写一个游戏，您可以在其中使用代币购买物品。 所有物品费用
// 5个代币，每当您购买物品时，都会产生1个手续费
// 令牌。 游戏玩家将输入他们想要购买的物品数量，然后
// `total_cost` 函数将计算代币的总成本。 自从
// 玩家输入数量，但我们将其作为字符串获取——而他们
// 可能输入了任何内容，而不仅仅是数字！
//
// 现在，这个函数根本不处理错误情况（并且不是
// 正确处理成功案例）。 我们想要做的是：如果我们调用
// 对非数字字符串的“parse”函数，该函数将
// 返回一个 `ParseIntError`，在这种情况下，我们希望立即返回
// 这个错误来自我们的函数，而不是尝试乘法和加法。
//
// 至少有两种实现此目的的方法都是正确的——但只有一种
// 短了很多！
//
// 执行 `rusdlingshinterrors2` 或使用 `hint`watch 子命令
// 暗示。


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    //let qty = item_quantity.parse::<i32>();

   // Ok(qty * cost_per_item + processing_fee)
    match item_quantity.parse::<i32>() {
        Ok(qty) => Ok(qty * cost_per_item + processing_fee),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
