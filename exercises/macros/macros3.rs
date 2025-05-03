// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// 使用 #[macro_use] 属性导出宏
// #[macro_use]
// mod macros {
//     macro_rules! my_macro {
//         () => {
//             println!("Check out my macro!");
//         };
//     }
// }

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro; // 将宏导出到模块外
}

fn main() {
    macros::my_macro!();
}
