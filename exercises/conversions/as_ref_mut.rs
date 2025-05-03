// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// Obtain the number of bytes (not characters) in the given argument.
// Add the AsRef trait appropriately as a trait bound.
// 添加 AsRef<str> 约束:使函数同时接受 &str 和 String 类型,AsRef<str> 是处理字符串参数的标准，兼容所有字符串类类型
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// Add the AsRef trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// Add the appropriate trait bound.
// 添加 AsMut<u32> 约束:支持任何可 mut 解引用为 u32 的类型(例: Box<u32>、&mut u32)
// AsMut<u32> 允许函数操作多种智能指针(例: Box/Rc/Arc 的可变版本)
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    let num = arg.as_mut();
    *num *= *num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
