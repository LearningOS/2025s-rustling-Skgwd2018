// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

// 宏规则分隔要求：
// 在 macro_rules! 中，每个匹配分支(称为"arm")必须用分号 ; 分隔
// 这类似于 match 表达式用逗号分隔分支，但宏规则要求用分号
// Rust 的宏解析器需要明确知道一个分支在哪里结束
// 缺少分隔符会导致解析错误(no rules expected this token)
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
