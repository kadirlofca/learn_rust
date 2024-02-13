use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

pub fn panic_button() {
    panic!("crash and burn");
}

fn recoverable_errors() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    // We can use unwrap_or_else too.
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Unwrap unwraps a Result value if it's valid, and panics if it is not valid.
    let greeting_file = File::open("hello.txt").unwrap();
}

fn propagating_errors() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e), // Here we return error back into the code that calls this function.
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // ? operator as shortcut to error propagation.
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file2 = File::open("hello.txt")?;
        let mut username2 = String::new();
        username_file2.read_to_string(&mut username2)?; // If Result is ok, we assign the ok value, if Result is err, we return the error.
                                                        // ? operator uses the From function to convert Result's into the type specified in the function return type.
        Ok(username2)
    }

    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username3 = String::new();

        File::open("hello.txt")?.read_to_string(&mut username3)?; // We can chain ? operator too!

        Ok(username3)
    }
}

fn propagating_options() {
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last() // Now we're using ? operator to return an Option.
    }
}

// Panic or not Panic? https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html