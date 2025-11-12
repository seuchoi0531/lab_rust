fn example() -> i32 {
    let x = 42;
    let v = if x < 42 { -1 } else { 1 };
    println!("if로 부터: {}", v);

    let food = "햄버거";
    let result = match food {
        "핫도그" => "핫도그다",
        _ => "핫도그 아니다",
    };
    println!("match로 부터: {}", result);

    let v = {
        let a = 4;
        let b = 11;
        a + b
    };
    println!("block으로 부터: {}", v);

    let y = 10;
    let rst = match y {
        0..=10 => "10이하",
        11.. => "10 초과",
        _ => "그럴 순 없음",
    };
    println!("{}", rst);

    v + 4
}
fn main() {
    println!("function로 부터: {}", example());
}
