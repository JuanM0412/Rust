use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn read_usrname_from_file1() -> Result<String, io::Error> {
    let f = File::open("username.txt");

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

fn read_usrname_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_usrname_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    
    File::open("username.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // Unrecoverable Error with panic!
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem openning the file: {:?}",
                error
            )
        },
    };

    let f = File::open("test.txt").unwrap();

    let f = File::open("file.txt").expect("Failed to open file.txt");
}
