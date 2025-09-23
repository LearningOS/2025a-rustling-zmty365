// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);  // 传递引用，不获取所有权

    string_uppercase(data);  // 现在可以正常获取所有权
}

// 应该借用而不是获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应该获取所有权并实际修改数据
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();  // 需要重新赋值，因为 to_uppercase() 返回新字符串

    println!("{}", data);
}
