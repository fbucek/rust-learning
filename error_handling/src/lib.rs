use std::io::Read;
use std::*;

pub fn read_from_file() -> Result<String, io::Error> {
    let mut f = fs::File::open("Cargo.toml")?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    Ok(text)
}

pub fn result_function(filename: &str) -> bool {
    let f: result::Result<std::fs::File, std::io::Error> = fs::File::open(&filename);

    // Now I can reprocces error and packi again into same Result
    let f: result::Result<std::fs::File, std::io::Error> = match f {
        Ok(file) => Ok(file),
        Err(error) => {
            eprintln!("Typeed let: Not possible to open file {:?}", error);
            Err(error)
            //panic!("Problem opening ifle: {:?}", error)
        },
    };

    // this would be for results
    if f.is_err() {
        return false;
    }
    return true;
}

pub fn option_function(filename: &str) -> bool {
  let f: result::Result<std::fs::File, std::io::Error> = fs::File::open(&filename);

    // Now I can reprocces error and packi again into same Result
    let f: result::Result<std::fs::File, std::io::Error> = match f {
        Ok(file) => Ok(file),
        Err(error) => {
            eprintln!("Typeed let: Not possible to open file {:?}", error);
            Err(error)
            //panic!("Problem opening ifle: {:?}", error)
        },
    };

    let f = match f {
        Ok(file) => Some(file),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match fs::File::create(&filename) {
                Ok(fc) => Some(fc),
                Err(e) => {
                    eprintln!("Problem creating file {:?}",e);
                    None
                }
            },
            other_error => {
                eprintln!("Problem opening file {:?}", other_error);
                None
            },
        },
    };
    
    if f.is_none() {
        return false;
    }

    // Now for sure f is std::fs::File
    return true;
}

pub fn panic_unwrap(filename: &str) -> bool {
    fs::File::open(&filename).unwrap();
    return true;
}

pub fn panic_expect(filename: &str) -> bool {
    fs::File::open(&filename).expect("Not possible to open file");
    return true;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panic_test() {
        // Unwrap
        assert_eq!(panic_unwrap(&"Cargo.toml"), true);
        let result = std::panic::catch_unwind(|| panic_unwrap(&"nonexisiting"));
        assert!(result.is_err()); 
        // Expect
        let res_expect = panic::catch_unwind(|| panic_expect(&"nonexisting"));
        assert!(res_expect.is_err());
    }


    #[test]
    fn result_test() {
        assert_eq!(result_function(&"Cargo.toml"), true);
        assert_eq!(result_function(&"nonexistent_Cargo.toml"), false);
    }

    #[test]
    fn option_test() {
        assert_eq!(option_function(&"Cargo.toml"), true);
        let nonexistfile = "xxxxxx.toml";
        assert_eq!(path::Path::new(&nonexistfile).exists(), false);
        assert_eq!(option_function(&nonexistfile), true);
        // Now have to cleanup
        assert!(fs::remove_file(&nonexistfile).is_ok());
    }
}

