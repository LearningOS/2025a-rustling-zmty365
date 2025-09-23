// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// 方法三，可变借用直接修改vec0
fn main() {
    let mut vec0 = Vec::new();

    // 传递 vec0 的可变引用给 fill_vec
    fill_vec(&mut vec0);

    // 克隆修改后的 vec0 给 vec1
    let mut vec1 = vec0.clone();

    println!(
        "{} has length {}, with contents: `{:?}`",
        "vec0",
        vec0.len(),
        vec0
    );

    vec1.push(88);

    println!(
        "{} has length {}, with contents `{:?}`",
        "vec1",
        vec1.len(),
        vec1
    );
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    // 不需要返回值，直接修改原向量
}
