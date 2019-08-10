fn multi_language() {
    let mut vec: Vec<String> = Vec::new();

    vec.push(String::from("السلام عليكم"));
    vec.push(String::from("Dobrý den"));
    vec.push(String::from("Hello"));
    vec.push(String::from("שָׁלוֹם"));
    vec.push(String::from("नमस्ते"));
    vec.push(String::from("こんにちは"));
    vec.push(String::from("안녕하세요"));
    vec.push(String::from("你好"));
    vec.push(String::from("Olá"));
    vec.push(String::from("Здравствуйте"));
    vec.push(String::from("Hola"));

    for str in &vec {
        println!("{}", str);
    }

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);

    // String lenght
    let hindi = "नमस्ते";
    println!("String lenght");
    println!("{} byte len is: {} bytes", hindi, hindi.len());
    println!("{} char len is: {:?} bytes", hindi, hindi.chars());
}

fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();
    println!("Hello, world!");

    multi_language();
}
