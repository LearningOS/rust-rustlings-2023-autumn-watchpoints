// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}


// 通过在代码中使用 #[macro_use] 属性，您可以全局地导入宏定义，而不仅仅是在一个特定的函数或模块中。
// 这有助于确保宏的定义在整个项目中都可用，而不会出现未使用宏的警告。