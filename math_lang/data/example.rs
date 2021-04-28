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
    let mut a = 5f64;
    let mut b = 0.0;
    b = input();
    println!("<output>: {}", a + b);
    let mut c = a + 5f64 * (1f64 + b);
    println!("<output>: {}", c);
    let mut d = 1f64 / ((a).sin() + 2f64.pow((b).sqrt() - 3f64)) % c;
    println!("<output>: {}", d);
    println!("<output>: {}", (d).log10());
    println!("<output>: {}", (2f64 - ((pi * e).sin()).abs() + (2f64 + (pi / 4f64).atanh()).ln()).floor());
}
