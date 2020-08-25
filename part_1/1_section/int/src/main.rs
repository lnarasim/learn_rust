fn print_integer(x: i8) {
    println!("Decimal {}", x);
    println!("Hexa {:0X}", x);
    println!("Octal {:0o}", x);
    println!("Binary {:0b}", x);
}

fn underflow() {
    print!("--------------Underflow-------------------");
    let mut x:i8 = 1;
    for _i in 1..1234 {
        print!("{}", x);
        x = x - 1;
    }
}
fn overflow() {
    print!("--------------Overflow-------------------");
    let mut x:i8 = 1;
    for _i in 1..123254 {
        println!("{}", x);
        x = x + 1;
    }
}
fn main() {
    let mut signed: i8 = 23;
    print_integer(signed);

    signed = -23;
    print_integer(signed);

    let smilie = '😻';
    println!("{}", smilie);

    overflow();
    underflow();
}
