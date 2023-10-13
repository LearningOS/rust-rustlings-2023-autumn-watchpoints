// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // let cost = total_cost(pretend_user_input)?;

    // if cost > tokens {
    //     println!("You can't afford that many!");
    // } else {
    //     tokens -= cost;
    //     println!("You now have {} tokens.", tokens);
    // }
    match total_cost(pretend_user_input) {
        Ok(cost) => {
            if cost > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= cost;
                println!("You now have {} tokens.", tokens);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?; 
    //`?` 操作符的作用是检查`Result`的值。如果`Result`是 `Ok`，它会从 `Ok` 包裹中提取成功的值并将其绑定到左边的变量（这里是 `qty`）。
    //如果`Result`是 `Err`，`?` 操作符会提前返回整个函数，并将 `Err` 包裹的错误值作为返回值，从而自动传播错误。

    Ok(qty * cost_per_item + processing_fee)
    
}
