use std::io::Write;

#[allow(dead_code)]
fn input() -> f64 {
    let mut text = String::new();
    eprint!("<input>: ");
    std::io::stderr().flush().unwrap();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    text.trim().parse::<f64>().unwrap_or(0.)
}


fn main() {
    let mut _a = 5f64;
    let mut _b = 0.0;
    _b = input();
    println!("<output>: {}", a + b);
    let mut _c = a + 5f64 * (1f64 + b);
    println!("<output>: {}", c);
    let mut _d = 1f64 / ((a).sin() + 2f64.pow((b).sqrt() - 3f64)) % c;
    println!("<output>: {}", d);
    println!("<output>: {}", (d).log10());
}
