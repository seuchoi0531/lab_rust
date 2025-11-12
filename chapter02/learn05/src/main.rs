fn main() {
    let x = 42;
    match x {
        0 => {
            println!("0 발견");
        }
        1 | 2 => {
            println!("1 또는 2 발견")
        }
        3..=9 => {
            println!("3부터 9까지의 숫자 발견")
        }
        a @ 10..=100 => {
            println!("10에서 100까지의 숫자 {} 발견!", a);
        }
        _ => {
            println!("다른거 발견");
        }
    }
}
