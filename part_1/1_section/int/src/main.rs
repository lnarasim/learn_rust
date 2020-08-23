fn print_integer(x: i8) {
    println!("Decimal {}", x);
    println!("Hexa {:0X}", x);
    println!("Octal {:0o}", x);
    println!("Binary {:0b}", x);
}


fn main() {
    let mut signed: i8 = 23;
    print_integer(signed);

    signed = -23;
    print_integer(signed);

    let smilie = '😻';
    println!("{}", smilie)
}
