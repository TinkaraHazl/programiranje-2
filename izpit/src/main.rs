fn f(c: u32) -> u32 {
    c + c
}
fn main() {
    let a = 10;
    let b = 20;
    let d = f(b) + a;
    println!("{d}");
}
