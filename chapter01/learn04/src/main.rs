fn main() {
    let x = 12;
    let a = 12u8;
    let b = 4.3;
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello, world!";
    // let array = [13, "as"];
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}
