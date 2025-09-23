// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into()); // 都行
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

// &str：字符串切片，通常是借用/引用
// String：可变的、拥有所有权的字符串

// 返回 String 的方法：
// .to_string() - 将 &str 转换为 String
// String::from() - 从 &str 创建 String
// .to_owned() - 创建拥有所有权的副本（&str → String）
// format!() - 格式化字符串，返回 String
// .replace() - 替换字符串内容，返回新的 String
// .to_lowercase() - 转换为小写，返回新的 String（因为可能改变长度）

// 返回 &str 的方法：
// 字符串字面量："blue" 本身就是 &str
// .trim() - 去除两端空白，返回原字符串的切片（&str）
// 字符串切片：&string[0..1] 返回原字符串的部分切片（&str）

// 特殊情况：
// .into() - 根据上下文进行类型转换，这里需要看期待的类型