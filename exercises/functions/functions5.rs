// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    //表达式会产生值，语句不会产生值。
    // 在 Rust 中，函数返回值可以通过显式使用 return 关键字或隐式使用最后一个表达式（不带分号）来指定。
    //
    // return num * num;
    num * num 
}
