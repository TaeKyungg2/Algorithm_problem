use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 실패");
    let num: i32 = input.trim().parse().expect("숫자가 아닙니다");
    println!("{}", num*(num-1));
}