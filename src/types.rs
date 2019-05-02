pub fn run() {
    let x: i64 = 45;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max u32: {}", std::u32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max u128: {}", std::u128::MAX);

    // Boolean
    let is_active = true;
    println!("{:?}", (x, is_active));
}
