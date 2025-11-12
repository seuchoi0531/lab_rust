fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "13 찾음!";
        }
    };
    println!("loop에서: {}", v);
}
