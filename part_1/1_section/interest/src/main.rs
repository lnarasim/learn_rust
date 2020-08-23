fn compound_interest(p: f64, n:f64, r:f64) -> f64 {
    let amount = p * (1.0 + r / 100.0 ).powf(n);
    amount - p
}

fn simple_interest(p: f64, n:f64, r:f64) -> f64 {
    p * n * r / 100.0
}


fn main() {
    println!("CI {}", compound_interest(100 as f64, 5 as f64, 5 as f64));
    println!("SI {}", simple_interest(100 as f64, 5 as f64, 5 as f64));
}