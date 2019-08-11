use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

#[allow(dead_code)]
fn panic_rust() {
    let v = vec![1, 3];
    println!("{:?}", v[40]); // <-- only 2 elements
}

fn better_open() {
    let filename = "hello_better.txt";

    let _f = File::open(&filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&filename).unwrap_or_else(|error| {
                panic!("Tried to create file but there was problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
            //println!("There was problem opening file {:?}", error);
        }
    });
}

#[allow(dead_code)]
fn expect_open() {
    let _f = File::open("hello_epect.txt").expect("Failed to open file hello_epect.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    println!("NOT VISIBLE IF NO FILE");
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("Content of file is: {}", &s.trim());
    Ok(s)
}

fn read_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    //
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // panic_rust(); <-- function causes panic

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!(
                    "Try to create file but there was problem
                    {:?}",
                    e
                ),
            },
            other_error => {
                panic!("There was a problem opening file: {:?}", other_error)
                //println!("no file {:?}", other_error);
                //println!("no file {}", other_error.raw_os_error().unwrap());
                //panic!("There was problem opening file: {:?}", error)
            }
        },
    };

    println!("{:?}", f);

    //better_open(); <-- panic

    //expect_open();

    read_username_from_file().unwrap();

    match read_username_short() {
        Ok(s) => println!("{}", s.trim()),
        Err(err) => println!("{}", err),
    }

    let s = read_username_short();
    if s.is_ok() {
        println!("Unwrapped s: {}", s.unwrap().trim());
    }
    //println!("File content is: {}", &s);
    read_even_shorter().unwrap();

    // shortest method is
    let s = fs::read_to_string("hello.txt");
    if s.is_ok() {
        println!("Shortes method: {}", s.unwrap().trim());
    }
}
