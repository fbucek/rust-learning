fn vecfunct() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // 
    println!("Third element is {}", third);
    println!("Third element is {}", third);

    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no such element"),
    }

    // Panic code
    //let does_not_exist: &i32 = v.get(100).unwrap();
    //println!("does_not_exist contains: {:?}", does_not_exist);
    //let does_not_exist = &v[100];

    let first = &v[0]; // <- problem with print after v.push(5);
    v.push(5);

    // first is used after vector could be changed -> v.push -> new allocation possible
    // -> new allocation would invalidate reference ( which is forbiden )
    // println!("The first element is: {}", first); 

}

fn iterating() {
    let mut v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
    
    for i in &v {
        println!("{}", i);
    }
}

fn vector_with_enum() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];  
}


fn main() {
    let mut v: Vec<i32> = Vec::new();

    for index in 0..4 {
        println!("index is: {index}", index = index);
        if index == 1 {
            break;
        }
    }

    v.push(5);
    println!("Vector {:?}", v);

    vecfunct();

    iterating();

    vector_with_enum();
}
