// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//您可以将模块路径引入范围并为它们提供新名称
//“use”和“as”关键字。 修复这些“use”语句以生成代码
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


//https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
mod delicious_snacks {
    // TODO: Fix these use statements
    //use self::fruits::PEAR as ???
    //use self::veggies::CUCUMBER as ???
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;


    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
