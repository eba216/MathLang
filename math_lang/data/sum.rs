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
    let mut _a = 0.0;
    _a = 5f64;
    let mut _b = 0.0;
    _b = input();
    println!("<output>: {}", _a + _b);
    let mut _c = 0.0;
    _c = _a + 5f64 * (1f64 + _b);
    println!("<output>: {}", _c);
    let mut _d = 0.0;
    _d = 1f64 / (_a + 2f64 * (_b - 3f64)) + _c;
    println!("<output>: {}", _d);
}
