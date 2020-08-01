fn main() {
    let mut x = 10;
    x = x + 1;
    println!("{}", &x);

    let y = &x;

    println!("{}", &x);
    println!("{}", &y);
}