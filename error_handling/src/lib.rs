use std::io::Read;
use std::*;
//use tempfile::tempdir;

pub fn read_from_file(filename: &str) -> Result<String, io::Error> {
    let mut f = fs::File::open(&filename)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    Ok(text)
}

pub fn read_text(filename: &str) -> Result<String, Box<dyn error::Error>> {
    Ok(read_from_file(&filename)?)
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
        }
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
            eprintln!("Typed let: Not possible to open file {:?}", error);
            Err(error)
            //panic!("Problem opening ifle: {:?}", error)
        }
    };

    let f = match f {
        Ok(file) => Some(file),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match fs::File::create(&filename) {
                Ok(fc) => Some(fc),
                Err(e) => {
                    eprintln!("Problem creating file {:?}", e);
                    None
                }
            },
            other_error => {
                eprintln!("Problem opening file {:?}", other_error);
                None
            }
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
    fn read_from_file_test() {
        // Must not panic and must read some bytes
        assert!(read_from_file(&"Cargo.toml").unwrap().len() > 0);
        // Must panic when unwrap
        let res = std::panic::catch_unwind(|| panic_expect(&"nonexisting"));
        assert!(res.is_err());

        let text = read_from_file(&"nonexisting").unwrap_or(String::new());
        assert_eq!(text.len(), 0);

        let text = read_from_file(&"nonexisting").unwrap_or_else(|err| {
            println!("printing unwrap or else parameter:{:?}", err);
            return String::new();
        });
        assert_eq!(text.len(), 0);

        let text = read_from_file(&"nonexisting").unwrap_or_else(|_| "".to_string());
        assert_eq!(text.len(), 0);
    }

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
        // Have to test file in temporary folder
        let filename = "Cargo.toml";
        assert_eq!(option_function(&filename), true);

        // Create a directory inside of `std::env::temp_dir()`.
        let dir = tempfile::tempdir().unwrap();
        let temppath = dir.path().join("xxxxxx.toml");
        let nonexistfile = &temppath.to_str().unwrap();
        assert_eq!(path::Path::new(&nonexistfile).exists(), false);
        assert_eq!(option_function(&nonexistfile), true);
        assert_eq!(path::Path::new(&nonexistfile).exists(), true);

        // Now have to cleanup
        assert!(fs::remove_file(&nonexistfile).is_ok());
        // At the end created file must not exists
        assert_eq!(path::Path::new(&nonexistfile).exists(), false);
    }
}
